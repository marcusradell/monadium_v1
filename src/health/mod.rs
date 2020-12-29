use actix_web::{web, HttpResponse, Responder};
use serde;
use std::sync::Mutex;

#[derive(serde::Serialize, Copy, Clone, Debug)]
pub enum Status {
    Live,
    Ready,
}

#[derive(serde::Serialize, Debug)]
pub struct Health {
    pub status: Mutex<Status>,
}

impl Health {
    pub fn live(&self) -> bool {
        true
    }

    pub fn ready(&self) -> bool {
        match *self.status.lock().unwrap() {
            Status::Live => false,
            Status::Ready => true,
        }
    }

    // pub fn mut_ready(&mut self) {
    //     let mut status = self.status.lock().unwrap();
    //     self.status = Mutex::new(Status::Ready);
    // }

    pub fn status(&self) -> Status {
        *self.status.lock().unwrap()
    }
}

async fn live(data: web::Data<Health>) -> impl Responder {
    match data.live() {
        false => HttpResponse::InternalServerError(),
        true => HttpResponse::Ok(),
    }
}

async fn ready(data: web::Data<Health>) -> impl Responder {
    match data.ready() {
        false => HttpResponse::InternalServerError(),
        true => HttpResponse::Ok(),
    }
}

async fn status(data: web::Data<Health>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&data.status()).unwrap())
}

async fn set_status(data: web::Data<Health>) -> impl Responder {
    // data.mut_ready();
    let mut status = data.status.lock().unwrap();
    *status = Status::Ready;
    println!("{:?}", status);
    HttpResponse::Ok()
}

pub fn schema(cfg: &mut web::ServiceConfig) {
    let data = web::Data::new(Health {
        status: Mutex::new(Status::Live),
    });

    cfg.service(
        web::scope("/health")
            .app_data(data.clone())
            .service(web::resource("/live").route(web::get().to(live)))
            .service(web::resource("/ready").route(web::get().to(ready)))
            .service(web::resource("/status").route(web::get().to(status)))
            .service(web::resource("/set_status").route(web::get().to(set_status))),
    );
}
