use crate::io::result::Result;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone)]
pub struct Deps {
    pub fake_db: String,
    pub fake_mq: String,
}

fn list(_id: Uuid, _fake_db: String) -> Result<()> {
    println!("List called.");
    Ok(())
}

fn create(_fake_db: String) -> Result<()> {
    println!("Create called.");
    Ok(())
}

impl Deps {
    pub fn config(&self, cfg: &mut web::ServiceConfig) {
        let db = self.fake_db.clone();
        let db2 = self.fake_db.clone();

        cfg.service(
            web::scope("/experiment")
                .route(
                    "/view/{id}",
                    web::get().to(move |args: web::Path<ViewArgs>| {
                        list(args.id, db.clone());
                        HttpResponse::Ok()
                    }),
                )
                .route(
                    "/create",
                    web::get().to(move || {
                        create(db2.clone());
                        HttpResponse::Ok()
                    }),
                ),
        );
    }
}

#[derive(Deserialize)]
struct ViewArgs {
    id: Uuid,
}
