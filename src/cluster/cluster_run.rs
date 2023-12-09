use actix_web::{post, web, HttpResponse, Responder};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
};

use crate::models::RunScrapers;
use crate::run_task::run_script;

#[post("/run")]
pub async fn run_scraper(
    incoming: web::Json<RunScrapers>,
) -> impl Responder {
    let RunScrapers { services, thread: threads_avi } = incoming.into_inner();

    let services = Arc::new(Mutex::new(VecDeque::from(services)));
    let finished_scrapers: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let failed_scrapers: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    
    let mut handles = vec![];
    for _ in 0..threads_avi {
        let services_clone = Arc::clone(&services);
        let finished_clone = Arc::clone(&finished_scrapers);
        let failed_clone = Arc::clone(&failed_scrapers);

        let handle = thread::spawn(move || {
            while let Some(service) = services_clone.lock().unwrap().pop_front() {
                match run_script(service.clone()) {
                        Ok(output) => {
                            let mut lock = if output.status.code().unwrap_or(-1) != 0 {
                                failed_clone.lock().unwrap()
                            } else {
                                finished_clone.lock().unwrap()
                            };
                            lock.push(service.clone());
                        }
                        Err(_) => {
                            failed_clone.lock().unwrap().push(service.clone());
                        }
                    }
            }

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let msg = serde_json::json!({
        "code": 200,
        "finished_scrapers": *finished_scrapers,
        "failed_scrapers": *failed_scrapers,
    });
    let response = HttpResponse::Ok().json(msg);
    response
}
