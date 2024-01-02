use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use env_logger::Env;
use log::info;
use std::env;

use sysinfo::{System, SystemExt, ProcessExt};

mod tasks;
use tasks::run_task;
mod models;
use models::Harmonogram;
mod cluster;
use cluster::cluster_run;
mod harmonogram;
use harmonogram::post_harmonogram::post_harmonogram;

//TODO: test save harmonogram -> POST harmonograms
// TODO Test save harmonogram and if its running autmoaticly

/// Entry point of the application responsible for initializing the server and services.
///
/// This function initializes environment variables.
/// starts the cron service, sets up CORS configuration, and begins the Actix HTTP server with the defined routes.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let thread_num = if num_cpus::get() > 6 {
        num_cpus::get() - 2
    } else {
        num_cpus::get()
    };

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenv::dotenv().ok();
    let api_url = env::var("HTTP_URL").expect("[ERROR] - Missing API_URL define or .env file!");
    
    start_cron_service();
    info!(
        "Starting server in {:?} with {:?} threads",
        api_url,
        thread_num
    );

    HttpServer::new(move || {
        let cors_front = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors_front)
            .service(cluster_run::run_scraper)
            .service(post_harmonogram)
    })
    .workers(thread_num)
    .bind(api_url)?
    .run()
    .await
}

fn is_cron_running_as_root()->bool {
    let system = System::new_all();
    system.processes().iter().any(|(_, proc)| {
        proc.name() == "cron" && if let Some(uid) = proc.user_id() {
         uid.to_string() == "0" 
        } else { false }
    })
}
/// Initiates the cron service in the background.
///
/// This function is responsible for starting the cron service as a background task.
/// It uses the `Command` struct from the standard library to execute the `cron` command.
fn start_cron_service() {
    if !is_cron_running_as_root() {
        Command::new("cron")
            .arg("-f")
            .spawn()
            .expect("[ERROR] - Failed to start cron service");
        info!("Cron service start as root.")
    } else {
        info!("Cron service is already running as root.")
    }
}

use std::process::Command;
use std::fs::File;
use std::io::Write;

/// Writes the harmonogram entries as cron jobs to a file and loads them into the cron service.
///
/// This function processes the harmonogram entries, converts them to cron job format, and writes them
/// to a specified file. Subsequently, it loads these cron jobs into the cron service.
fn save_cron_to_file(harmonograms: Vec<Harmonogram>, port: String, endpoint_to_run: String) -> std::io::Result<String> {
    if harmonograms.len() == 0 { return Ok("[ERROR] - Empty Harmonogram".to_string())}

    let mut cron_content = String::new();
    for harmonogram in harmonograms.iter() {
        let cron = serde_json::json!({
            "thread": harmonogram.thread,
            "services": harmonogram.scrapers
        });

        let cron_str = cron.to_string();
        let single_cron: String = format!(
            "{} curl --location http://127.0.0.1:{}{} --header \"Content-Type: application/json\" --data '{}'\n",
            harmonogram.time, port, endpoint_to_run, cron_str
        );

        cron_content.push_str(&single_cron);
    }

    let mut file = File::create("harmonogram_cron_file")?;
    file.write_all(cron_content.as_bytes())?;

    Command::new("crontab")
        .arg("harmonogram_cron_file")
        .output()
        .expect("Failed to execute command");

    Ok("[INFO] - Load Harmonograms to file".to_string())
}