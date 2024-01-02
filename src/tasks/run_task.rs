use std::{process::{Command, Output}, env};

/// Executes a script for a given parameters using the program command.
///
/// This function runs a specific program located in the user's home directory under the 'JOBS_DIR/' path.
/// It takes a `task` string as an argument, and then constructs the appropriate app command to be executed.
/// It then returns the output status of the executed command.
pub fn run_script(task: String) -> std::io::Result<Output> {
    let home_dir = dirs::home_dir().expect("Could not get home directory.");

    dotenv::dotenv().ok();
    let jobs_dir = env::var("JOBS_DIR").expect("Could not get jobs directory.");
    let program_command = env::var("PROGRAM_COMMAND").expect("Could not get program_command from env.");
    let program_args: Vec<String> = env::var("PROGRAM_ARGS")
        .expect("Could not get PROGRAM_ARGS from env.")
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let target_dir = home_dir.join(jobs_dir);

    println!("home_dir: {:?}", home_dir);
    println!("target: {:?}", target_dir);
    println!("program_command: {:?}", program_command);
    println!("PROGRAM_ARGS: {:?}", program_args);

    let status = Command::new(&program_command)
        .current_dir(&target_dir)
        .args(&program_args)
        .arg(&task)
        .output()?;

    Ok(status)
}