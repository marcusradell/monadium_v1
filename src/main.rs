extern crate argon2;
extern crate dotenv;

use dotenv::dotenv;
mod domain;
mod io;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Booting server.");
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let jwt = io::jwt::Jwt::new();

    let uri = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL.");

    // TODO: Deprecate or merge with how the identities_repo work now.
    let es = io::event_store::EventStore::new(&uri).await.unwrap();
    let db = es.db().clone();
    // TODO: This should replace the EventStore.
    let identities_repo = domain::identities::repo::Repo::new(&db);

    let deps_experiment = crate::domain::deps_experiment::Deps {
        fake_db: "db".into(),
        fake_mq: "mq".into(),
    };

    let http = io::http::init(
        8080,
        jwt,
        db,
        identities_repo,
        vec![
            domain::health::config,
            domain::profiles::config,
            domain::identities::config,
        ],
        deps_experiment,
    )
    .await?;

    http.server.await
}
