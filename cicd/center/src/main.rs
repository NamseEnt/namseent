mod config;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    upload_source().await?;

    // for cicd_project in find_all_cicd_projects() {
    //     cicd_project.run();
    // }

    Ok(())
}

async fn upload_source() -> Result<()> {
    todo!()
}
