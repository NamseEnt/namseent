use anyhow::Result;
use std::{io::Write, path::Path};

pub async fn start_remote_run(
    remote_ip: &str,
    directory: &Path,
    executable_name: &str,
) -> Result<()> {
    let tar_gz = write_tar_gz(directory)?;

    reqwest::Client::new()
        .post(format!("http://{}:8986/start_remote_run", remote_ip))
        .header("exetuable-name", executable_name)
        .body(tar_gz)
        .send()
        .await?;

    Ok(())
}

fn write_tar_gz(directory: &Path) -> Result<Vec<u8>> {
    let mut tar = tar::Builder::new(vec![]);

    for entry in walkdir::WalkDir::new(directory) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            tar.append_path_with_name(path, path.file_name().unwrap())?;
        }
    }

    let mut gz = flate2::write::GzEncoder::new(vec![], flate2::Compression::default());
    gz.write_all(&tar.into_inner()?)?;
    Ok(gz.finish()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    #[tokio::test]
    async fn test_write_tar_gz() {
        let temp_dir = std::env::temp_dir();
        let root = temp_dir.join("remote-develop-agent-test_write_tar_gz");
        if root.exists() {
            std::fs::remove_dir_all(&root).unwrap();
        }
        std::fs::create_dir_all(&root).unwrap();
        let file_path = root.join("test.txt");
        {
            let mut file = std::fs::File::create(&file_path).unwrap();
            file.write_all("Hello".as_bytes()).unwrap();
            file.flush().unwrap();
        }
        {
            let mut file = std::fs::File::open(&file_path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            assert_eq!(content, "Hello");
        }

        let buffer = write_tar_gz(&root).unwrap();

        let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(&buffer[..]));

        for entry in tar.entries().unwrap() {
            let mut entry = entry.unwrap();

            assert_eq!(entry.path().unwrap(), std::path::Path::new("test.txt"));
            assert_eq!(entry.size(), 5);

            let mut content = String::new();
            let length = entry.read_to_string(&mut content).unwrap();
            assert_eq!(length, 5);
            assert_eq!(content, "Hello");
        }
    }

    #[tokio::test]
    async fn test_start_remote_run() {
        let temp_dir = std::env::temp_dir();
        let root = temp_dir.join("remote-develop-agent-test_start_remote_run");
        if root.exists() {
            std::fs::remove_dir_all(&root).unwrap();
        }
        std::fs::create_dir_all(&root).unwrap();
        let file_path = root.join("test.sh");
        let mut file = std::fs::File::create(file_path).unwrap();
        file.write_all(
            "#!/bin/bash
            echo Hello > target/output.txt"
                .as_bytes(),
        )
        .unwrap();

        let mut agent_command = std::process::Command::new("cargo")
            .args(["run"])
            .current_dir("../agent")
            .stdout(std::process::Stdio::null())
            .spawn()
            .unwrap();

        let mut retry = 0;
        loop {
            retry += 1;
            if retry > 30 {
                agent_command.kill().unwrap();
                panic!("Agent not started");
            }
            if reqwest::get("http://127.0.0.1:8986/health").await.is_ok() {
                break;
            }

            println!("Waiting for agent to start");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        start_remote_run("127.0.0.1", &root, "test.sh")
            .await
            .unwrap();

        let mut retry = 0;
        loop {
            retry += 1;
            if retry > 30 {
                agent_command.kill().unwrap();
                panic!("Output file not found");
            }

            if let Ok(content) = std::fs::read_to_string(Path::new("../agent/target/output.txt")) {
                assert_eq!(content, "Hello\n");
                agent_command.kill().unwrap();
                return;
            }

            println!("Waiting for output file");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
