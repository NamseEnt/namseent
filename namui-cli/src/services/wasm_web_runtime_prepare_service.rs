use crate::util::get_cli_root_path;

pub fn preapre() -> Result<(), crate::Error> {
    install_deps()?;

    build_browser_runtime()?;

    Ok(())
}

fn install_deps() -> Result<(), crate::Error> {
    let mut cmd = std::process::Command::new("npm");
    cmd.arg("i");
    cmd.current_dir(get_cli_root_path().join("webCode"));

    let output = cmd.output().unwrap();

    if !output.status.success() {
        return Err(format!(
            "Failed to install dependencies: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}

fn build_browser_runtime() -> Result<(), crate::Error> {
    let mut cmd = std::process::Command::new("npm");
    cmd.arg("run");
    cmd.arg("build");
    cmd.current_dir(get_cli_root_path().join("webCode"));

    let output = cmd.output().unwrap();

    if !output.status.success() {
        return Err(format!(
            "Failed to build browser runtime: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}
