use anyhow::anyhow;
use cnctd_shell::Shell;

pub struct Go;

impl Go {
    pub async fn check_for_go() -> Result<(), anyhow::Error> {
        match Shell::run_with_exit_status("go version", false).await? {
            0 => Ok(()),
            _ => Err(anyhow!("Go not installed"))
        }
    }
    
    pub async fn init(project_name: &str) -> anyhow::Result<()> {
        Self::check_for_go().await?;
        let command = format!("go mod init {}", project_name.to_lowercase().replace("_", "-").replace(" ", "-"));
        Shell::run(&command, false).await?;
        Ok(())
    }

    pub async fn install_package(package_name: &str) -> anyhow::Result<()> {
        let command = format!("go get {}", package_name);
        Shell::run(&command, true).await?;
        Ok(())
    }
       
}