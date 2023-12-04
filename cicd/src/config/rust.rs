use anyhow::Result;
use aws_sdk_ec2::error::ProvideErrorMetadata;
use std::path::Path;

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

    run_worker(path, config).await?;

    write_digest_cache(&digest).await?;

    Ok(())
}

pub(crate) struct WorkerContext<'a> {
    git_root_path: &'a Path,
    project_path: &'a Path,
}

async fn run_worker(context: &WorkerContext<'_>, config: &RustConfig) -> Result<()> {
    static EC2_CLIENT: tokio::sync::OnceCell<aws_sdk_ec2::Client> =
        tokio::sync::OnceCell::const_new();

    let ec2_client = EC2_CLIENT
        .get_or_init(|| async {
            let aws_config =
                aws_config::load_defaults(aws_config::BehaviorVersion::v2023_11_09()).await;
            aws_sdk_ec2::Client::new(&aws_config)
        })
        .await;

    // vpc랑 security group이 필요하다.

    let security_group_name = "rust-cicd";

    let security_group_id = match ec2_client
        .create_security_group()
        .group_name(security_group_name)
        .send()
        .await
    {
        Ok(ok) => ok.group_id().unwrap().to_string(),
        Err(err) => {
            if err.code() != Some("InvalidGroup.Duplicate") {
                return Err(err.into());
            }

            ec2_client
                .describe_security_groups()
                .group_names(security_group_name)
                .send()
                .await?
                .security_groups
                .unwrap()
                .first()
                .unwrap()
                .group_id
                .as_ref()
                .unwrap()
                .clone()
        }
    };

    let source_upload = upload_source_to_s3(&context.git_root_path).await?;

    let bash_script = format!(
        "#!/bin/bash
        set -e
        
        shutdown -h +{timeout_minutes}
        rustup toolchain install stable
        
        cd ~/
        mkdir app && cd app
        {source_download_command}
        {target_cache_download_command}
        cd {project_relative_path}
        
        cargo fmt --check
        cargo check
        cargo clippy
        cargo test

        {target_cache_upload_command} || true
    ",
        timeout_minutes = config.timeout_minutes.unwrap_or(15),
        source_download_command = source_upload.download_command(),
        target_cache_download_command = download_target_cache_command(context),
        project_relative_path = project_relative_path(context),
        target_cache_upload_command = upload_target_cache_command(context),
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
        .ipv6_address_count(1)
        .security_group_ids(security_group_id)
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

fn project_relative_path<'a>(context: &WorkerContext<'a>) -> &'a str {
    todo!()
}

fn download_target_cache_command<'a>(context: &WorkerContext<'a>) -> &'a str {
    todo!()
}

fn upload_target_cache_command<'a>(context: &WorkerContext<'a>) -> &'a str {
    todo!()
}

struct SourceUpload {}
impl SourceUpload {
    fn download_command(&self) -> &str {
        todo!()
    }
}

async fn upload_source_to_s3(git_root_path: &Path) -> Result<SourceUpload> {
    todo!()
}

async fn calculate_digest(path: &Path) -> Result<Digest> {
    todo!()
}

async fn is_digest_cache_hit(digest: &Digest) -> Result<bool> {
    todo!()
}

async fn write_digest_cache(digest: &Digest) -> Result<()> {
    todo!()
}

struct Digest {
    file_count: u16,
    file_size_crc: u32,
    file_content_crc: u32,
}
