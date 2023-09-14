use actix_web::{get, web, App, HttpServer, HttpResponse};
use serde_derive::{Deserialize};
use serde_json::json;
use chrono::prelude::{DateTime, Utc, Local, Datelike};

#[derive(Debug, Deserialize)]
pub struct APIQueryParams {
    slack_name: String,
    track: String
}

#[get("/api")]
async fn person_details(query_params: web::Query<APIQueryParams>) -> HttpResponse {
    let local: DateTime<Local> = Local::now();
    let day_index = local.weekday().num_days_from_sunday();

    let day_name = match day_index {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => "Invalid",
    };

    let raw_utc_time: DateTime<Utc> = Utc::now();
    let utc_time = raw_utc_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    
    let returned_data = json!({
        "slack_name": query_params.slack_name,
        "current_day": day_name,
        "utc_time": utc_time,
        "track": query_params.track,
        "github_file_url": "https://github.com/vicradon/hngx-task1/blob/main/main.py",
        "github_repo_url": "https://github.com/vicradon/hngx-task1",
        "status_code": 200
    });

    HttpResponse::Ok().json(returned_data)
}

#[get("/")]
async fn root() -> HttpResponse  {
    HttpResponse::Ok().body("It works!")
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(root).service(person_details)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

