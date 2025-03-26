use eyre::{bail, Context};
use std::path::Path;
use std::fs;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!("root: {:?}", root);
    
    // Ensure mofa_search_agent directory exists
    let agent_dir = root.join("mofa_search_agent");
    if !agent_dir.exists() {
        println!("Creating mofa_search_agent directory...");
        fs::create_dir_all(&agent_dir)?;
    }

    enter_dataflow().await?;
    build_dataflow().await?;
    start_dataflow().await?;
    Ok(())
}

async fn enter_dataflow() -> eyre::Result<()> {
    std::env::set_current_dir("./mofa_search_agent")
        .wrap_err("failed to change directory to mofa_search_agent")?;
    Ok(())
}

async fn build_dataflow() -> eyre::Result<()> {
    let dora = std::env::var("DORA")
        .wrap_err("DORA environment variable not set")?;
    
    let mut cmd = tokio::process::Command::new(&dora);
    cmd.arg("build").arg("dataflow.yml");
    
    let status = cmd.status().await
        .wrap_err("failed to execute dora build command")?;
        
    if !status.success() {
        bail!("dora build command failed");
    }
    
    Ok(())
}

async fn start_dataflow() -> eyre::Result<()> {
    let dora = std::env::var("DORA")
        .wrap_err("DORA environment variable not set")?;
    
    let mut cmd = tokio::process::Command::new(&dora);
    cmd.arg("start").arg("dataflow.yml");
    
    let status = cmd.status().await
        .wrap_err("failed to execute dora build command")?;
        
    if !status.success() {
        bail!("dora start command failed");
    }
    
    Ok(())
}


