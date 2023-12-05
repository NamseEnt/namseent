use anyhow::Result;
use futures::{StreamExt, TryStreamExt};
use std::path::{Path, PathBuf};
use tokio::{
    fs::{self, DirEntry},
    io::AsyncReadExt,
    sync::mpsc::UnboundedReceiver,
};

lazy_static::lazy_static! {
    static ref SUBNET_ID: String = std::env::var("RUST_CICD_SUBNET_ID").unwrap();
    static ref SECURITY_GROUP_ID: String = std::env::var("RUST_CICD_SECURITY_GROUP_ID").unwrap();
    static ref S3_BUCKET_NAME: String = std::env::var("CICD_S3_BUCKET_NAME").unwrap();
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct RustConfig {
    /// Default: 15 minutes
    timeout_minutes: Option<u16>,
    arch: RustArch,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) enum RustArch {
    #[serde(rename = "x86_64")]
    X86_64,
    #[serde(rename = "aarch64")]
    Aarch64,
}

/*
시스템은 2개로 나눔
- github actions에서 돌아가는 center
- ec2에서 돌아가는 worker
  - timeout이 존재하고, 무조건 그 이상 지나면 강제종료함.
  - center와 ping이 끊겨도 강제종료함.

프로젝트 digest 구해서 이미 통과된 것인지 아닌지 파악하기. : center에서 계산함.
digest는 3개로 이루어짐. 파일 갯수, 파일 크기, 파일 내용.
캐시는 s3에서.

해야하는건 cargo fmt, check, clippy, test

빌드 후 배포해야하는 녀석은?
빌드 할지 말지도 정하게 해주고, 빌드 결과물을 다른 cicd 스텝에서 가져다 쓸 수 있게 하면?

*/

async fn run(path: &Path, config: &RustConfig) -> Result<()> {
    println!("Running Rust project with config: {:?}", config);

    let digest: Digest = calculate_digest(path).await?;

    if is_digest_cache_hit(&digest).await? {
        println!("Digest cache hit!");
        return Ok(());
    }

    run_worker(todo!(), config).await?;

    write_digest_cache(&digest).await?;

    Ok(())
}

pub(crate) struct WorkerContext {
    pub project_path: String,
    source_targz_s3_key: String,
}

impl WorkerContext {
    fn yum_install_dependencies_bash_command(&self) -> &str {
        "yum install -y tar pigz"
    }
    fn download_source_bash_command(&self) -> String {
        format!(
            "aws s3 cp s3://{s3_bucket_name}/{source_targz_s3_key} - | pigz -d | tar -xvf -",
            s3_bucket_name = S3_BUCKET_NAME.as_str(),
            source_targz_s3_key = self.source_targz_s3_key,
        )
    }

    fn download_cache_bash_command(&self, key: impl AsRef<str>) -> String {
        format!(
            "aws s3 cp s3://{s3_bucket_name}/{key} - | pigz -d | tar -xvf -",
            s3_bucket_name = S3_BUCKET_NAME.as_str(),
            key = key.as_ref(),
        )
    }

    fn upload_cache_bash_command<'a>(
        &self,
        key: impl AsRef<str>,
        paths: impl IntoIterator<Item = &'a str>,
    ) -> String {
        let paths = paths.into_iter().collect::<Vec<_>>().join(" ");
        format!(
            "tar -cvf - {paths} | pigz | aws s3 cp - s3://{s3_bucket_name}/{key}",
            key = key.as_ref(),
            s3_bucket_name = S3_BUCKET_NAME.as_str(),
        )
    }
}

async fn run_worker(context: &WorkerContext, config: &RustConfig) -> Result<()> {
    static EC2_CLIENT: tokio::sync::OnceCell<aws_sdk_ec2::Client> =
        tokio::sync::OnceCell::const_new();

    let ec2_client = EC2_CLIENT
        .get_or_init(|| async {
            let aws_config =
                aws_config::load_defaults(aws_config::BehaviorVersion::v2023_11_09()).await;
            aws_sdk_ec2::Client::new(&aws_config)
        })
        .await;

    let cache_key = format!(
        "rust-cicd/target-cache/{project_path}-{arch}",
        project_path = context.project_path,
        arch = match config.arch {
            RustArch::X86_64 => "x86_64",
            RustArch::Aarch64 => "arm64",
        }
    );

    let bash_script = format!(
        "#!/bin/bash
        set -e

        {yum_install_dependencies_bash_command}
        
        shutdown -h +{timeout_minutes}
        rustup toolchain install stable
        
        cd ~/
        mkdir app && cd app
        {source_download_command}
        {target_cache_download_command}
        cd {project_path}
        
        cargo fmt --check
        cargo check
        cargo clippy
        cargo test

        {target_cache_upload_command} || true
    ",
        yum_install_dependencies_bash_command = context.yum_install_dependencies_bash_command(),
        timeout_minutes = config.timeout_minutes.unwrap_or(15),
        source_download_command = context.download_source_bash_command(),
        target_cache_download_command = context.download_cache_bash_command(&cache_key),
        project_path = context.project_path,
        target_cache_upload_command = context.upload_cache_bash_command(&cache_key, ["target"]),
    );

    ec2_client
        .run_instances()
        .image_id(format!(
            "resolve:ssm:/aws/service/ \
            ami-amazon-linux-latest/ \
            al2023-ami-minimal-kernel-default-{arch}",
            arch = match config.arch {
                RustArch::X86_64 => "x86_64",
                RustArch::Aarch64 => "arm64",
            }
        ))
        .instance_type(match config.arch {
            RustArch::X86_64 => aws_sdk_ec2::types::InstanceType::C6iLarge,
            RustArch::Aarch64 => aws_sdk_ec2::types::InstanceType::C7gLarge,
        })
        .security_group_ids(SECURITY_GROUP_ID.as_str())
        .subnet_id(SUBNET_ID.as_str())
        .user_data(format!(
            "#!/bin/bash
                
            cat <<EOF > /tmp/run.sh
            {bash_script}
            
            EOF
            
            chmod +x /tmp/run.sh
            /tmp/run.sh
            if [ $? -ne 0 ]; then
                shutdown -h now
            fi
            ",
        ))
        .tag_specifications(
            aws_sdk_ec2::types::TagSpecification::builder()
                .resource_type(aws_sdk_ec2::types::ResourceType::Instance)
                .tags(
                    aws_sdk_ec2::types::Tag::builder()
                        .key("Name")
                        .value("rust-cicd-worker")
                        .build(),
                )
                .build(),
        )
        .send()
        .await?;

    todo!()
}

async fn calculate_digest(path: &Path) -> Result<Digest> {
    let mut file_count = 0;
    const CRC_32_ISCSI: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISCSI);
    let mut file_size_crc = CRC_32_ISCSI.digest();
    let mut file_content_crc = CRC_32_ISCSI.digest();

    for file in visit_dirs(path).await? {
        let metadata = file.metadata().await?;

        file_count += 1;
        file_size_crc.update(&metadata.len().to_le_bytes());
        file_content_crc.update(&fs::read(file.path()).await?);
    }

    Ok(Digest {
        file_count,
        file_size_crc: file_size_crc.finalize(),
        file_content_crc: file_content_crc.finalize(),
    })
}

async fn is_digest_cache_hit(digest: &Digest) -> Result<bool> {
    todo!()
}

async fn write_digest_cache(digest: &Digest) -> Result<()> {
    todo!()
}

async fn visit_dirs(dir: &Path) -> Result<Vec<fs::DirEntry>> {
    let mut dir_paths = vec![dir.to_path_buf()];
    let mut non_dir_entries = Vec::new();

    while let Some(dir_path) = dir_paths.pop() {
        let mut dir = fs::read_dir(dir_path).await?;
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                dir_paths.push(path);
            } else {
                non_dir_entries.push(entry);
            }
        }
    }

    Ok(non_dir_entries)
}

struct Digest {
    file_count: u16,
    file_size_crc: u32,
    file_content_crc: u32,
}
