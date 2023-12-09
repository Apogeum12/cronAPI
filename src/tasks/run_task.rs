use std::process::{Command, Output};

/// Executes a script for a given promotor using the npm command.
///
/// This function runs a specific npm script located in the user's home directory under the 'docker/scraping/prod/' path.
/// It takes a `promotor` string as an argument, and then constructs the appropriate npm command to be executed.
/// It then returns the output of the executed command.
pub fn run_script(promotor: String) -> std::io::Result<Output> {
    let home_dir = dirs::home_dir().expect("Could not get home directory.");
    let target_dir = home_dir.join("witan-scraper-scraping/backend/"); //? Uncomment when locally debugging

    let promotor_arg = format!("promotor={}", promotor);

    let status = Command::new("npm")
        .current_dir(target_dir)
        .arg("run")
        .arg("start")
        .arg(&promotor_arg)
        .output()?;

    Ok(status)
}
