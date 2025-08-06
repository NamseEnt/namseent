use crate::update_server_port;
use anyhow::Result;
use std::{
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};
use tokio::{process::Command, runtime::Handle, *};

const SERVER_DIR: &str = "/namseent/luda-editor-new-server/server";

// assume that server in-memory cache is disabled by default

pub fn keep_server_updated() {
    task::spawn(async move {
        let mut server: Option<Server> = None;

        loop {
            if let Some(inner) = server.as_mut() {
                if inner.is_process_exited().await {
                    server.take();
                    update_server_port(0);
                }
            }

            if let Err(err) = keep_server_updated_tick(&mut server).await {
                eprintln!("Failed to pull: {err}");
            }
            time::sleep(Duration::from_secs(10)).await;
        }
    });
}

async fn keep_server_updated_tick(server: &mut Option<Server>) -> Result<()> {
    git_pull().await?;
    let commit_hash = get_git_commit_hash().await;

    if server
        .as_ref()
        .is_some_and(|server| server.commit_hash == commit_hash)
    {
        return Ok(());
    }

    let new_server = Server::start().await?;
    if let Some(server) = server.as_ref() {
        server.turn_off_memory_cache().await?;
    }

    update_server_port(new_server.port);

    {
        server.replace(new_server);
    }

    if let Err(err) = server.as_ref().unwrap().turn_on_memory_cache().await {
        eprintln!("Failed to turn on memory cache: {err}");
        server.take();
        return Err(err);
    };

    Ok(())
}

async fn get_git_commit_hash() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(SERVER_DIR)
        .output()
        .await
        .unwrap();

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn next_port() -> u16 {
    static PORT: OnceLock<Arc<Mutex<u16>>> = OnceLock::new();
    let mut port = PORT
        .get_or_init(|| Arc::new(Mutex::new(8000)))
        .lock()
        .unwrap();
    let ret = *port;
    *port += 1;
    if *port >= 9000 {
        *port = 8000;
    }
    ret
}

struct Server {
    port: u16,
    commit_hash: String,
    process: tokio::process::Child,
}

async fn force_kill_process(process: &mut tokio::process::Child) {
    for _ in 0..10 {
        if let Ok(Some(_)) = process.try_wait() {
            return;
        }
        let _ = process.kill().await;
        time::sleep(Duration::from_secs(1)).await;
    }

    panic!("Failed to kill server");
}

impl Drop for Server {
    fn drop(&mut self) {
        tokio::task::block_in_place(|| {
            Handle::current().block_on(async move {
                force_kill_process(&mut self.process).await;
            });
        });
    }
}
impl Server {
    async fn start() -> Result<Self> {
        let commit_hash = get_git_commit_hash().await;

        let port = next_port();
        let build_result = Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(SERVER_DIR)
            .spawn()?
            .wait()
            .await?;

        if !build_result.success() {
            return Err(anyhow::anyhow!("Failed to build"));
        }

        let mut process = Command::new("bash")
            .args(["./target/release/server"])
            .current_dir(SERVER_DIR)
            .env("PORT", port.to_string())
            .spawn()?;

        for _ in 0..15 {
            if let Ok(Some(_)) = process.try_wait() {
                return Err(anyhow::anyhow!("Server exited during startup"));
            }

            time::sleep(Duration::from_secs(1)).await;
            if health_check(port).await.is_ok() {
                return Ok(Self {
                    port,
                    commit_hash,
                    process,
                });
            }
        }

        for _ in 0..10 {
            if let Ok(Some(_)) = process.try_wait() {
                return Err(anyhow::anyhow!("Failed to start server"));
            }
            let _ = process.kill().await;
            time::sleep(Duration::from_secs(1)).await;
        }

        panic!("Failed to kill server");
    }

    async fn is_process_exited(&mut self) -> bool {
        for _ in 0..10 {
            match self.process.try_wait() {
                Ok(Some(_)) => return true,
                Ok(None) => return false,
                Err(_) => {
                    time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
        panic!("Failed to check if process exited");
    }

    async fn turn_off_memory_cache(&self) -> Result<()> {
        reqwest::get(format!(
            "http://localhost:{}/turn_off_memory_cache",
            self.port
        ))
        .await?
        .error_for_status()?;
        Ok(())
    }

    async fn turn_on_memory_cache(&self) -> Result<()> {
        reqwest::get(format!(
            "http://localhost:{port}/turn_on_memory_cache",
            port = self.port
        ))
        .await?
        .error_for_status()?;

        Ok(())
    }
}

async fn health_check(port: u16) -> Result<()> {
    reqwest::get(format!("http://localhost:{port}/health"))
        .await?
        .error_for_status()?;
    Ok(())
}

async fn git_pull() -> Result<()> {
    let output = Command::new("git")
        .args(["pull"])
        .current_dir(SERVER_DIR)
        .spawn()?
        .wait_with_output()
        .await?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to pull: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}
