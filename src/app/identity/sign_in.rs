use crate::io::db;
use actix_web::{web, Error, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct SignInArgs {
    pub email: String,
    pub password: String,
}

pub async fn sign_in(
    pool: web::Data<db::Pool>,
    args: web::Json<SignInArgs>,
) -> Result<HttpResponse, Error> {
    // TODO: use the database to lookup the email and then verify the password properly.
    let conn = pool.get().expect("Couldn't get DB connection from pool.");

    let verify_result = argon2::verify_encoded("$argon2i$v=19$m=4096,t=3,p=1$5eOOzBOVnIIN42VyvQC0p1nWvYESj14vHF3npjm4O8s$Fx0Rtyr3J2En8aECSV9laaD0Zsw/loAARHQ19zIpU1M", args.password.as_bytes()).map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    match verify_result {
        true => println!("You sent the 'test1' password."),
        false => println!("You didn't send the 'test1' password."),
    };

    Ok(HttpResponse::Ok().finish())
}
