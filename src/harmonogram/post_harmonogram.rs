use actix_web::{
    post,
    web,
    HttpResponse, Responder,
};
use std::env;
use serde_json::json;

use crate::save_cron_to_file;
use crate::Harmonogram;

/// Endpoint to receive and process new harmonograms.
///
/// This function accepts harmonograms in JSON format, saves them to a cron file for scheduled execution, 
/// and posts the harmonograms to another API (`DNET_API`). It provides comprehensive error handling and feedback.
#[post("/harmonogram")]
pub async fn post_harmonogram(incoming: web::Json<Vec<Harmonogram>>,
) -> impl Responder {
    let harmonograms  = incoming.into_inner();

    dotenv::dotenv().ok();
    let port = env::var("HTTP_PORT").expect("[ERROR] - Missing HTTP_PORT define or .env file!");
    let endpoint_to_run = env::var("CLUSTER_API_RUN").expect("[ERROR] - Missing CLUSTER_API_RUN define or .env file!");
    

    match save_cron_to_file(harmonograms.clone(), port, endpoint_to_run) {
        Ok(_) => {
            HttpResponse::Ok().json(json!({
                "code": 200,
                "msg": "Successful saved harmonogram to cron."
            }))
        },
        Err(e) => HttpResponse::Ok().json(json!({
            "code": 502,
            "msg": format!("Failed save harmonograms to file: {}", e)
        }))
    }
}