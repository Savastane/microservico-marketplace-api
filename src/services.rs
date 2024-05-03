use actix_web::{
    web::{ 
        scope, 
        ServiceConfig
    },
    get,
    HttpResponse,
    Responder
};
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "HealthChecker";

    HttpResponse::Ok().json(json!({
        "Status": "success",
        "message": MESSAGE
    })) 

}

pub fn  config(conf: &mut ServiceConfig) {
   
  let scope = scope("").service(health_checker);
  conf.service(scope);
}

