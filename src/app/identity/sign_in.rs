use crate::io::db;
use crate::io::jwt;
use actix_web::{web, Error, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct SignInArgs {
    email: String,
    password: String,
}

#[derive(serde::Serialize, Debug)]
struct SignInResponse {
    jwt: String,
}

pub async fn sign_in(
    pool: web::Data<db::Pool>,
    jwt: web::Data<jwt::Jwt>,
    args: web::Json<SignInArgs>,
) -> Result<HttpResponse, Error> {
    // TODO: use the database to lookup the email and then verify the password properly.
    let _conn = pool.get().expect("Couldn't get DB connection from pool.");

    let verify_result = argon2::verify_encoded("$argon2i$v=19$m=4096,t=3,p=1$5eOOzBOVnIIN42VyvQC0p1nWvYESj14vHF3npjm4O8s$Fx0Rtyr3J2En8aECSV9laaD0Zsw/loAARHQ19zIpU1M", args.password.as_bytes()).map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    match verify_result {
        true => {
            println!("You sent the 'test1' password.");
            let encoded_jwt = jwt.into_inner().encode(args.email.clone()).map_err(|e| {
                eprintln!("{}", e);
                return HttpResponse::InternalServerError().finish();
            })?;
            Ok(HttpResponse::Ok().json(SignInResponse { jwt: encoded_jwt }))
        }
        false => {
            println!("You didn't send the 'test1' password.");
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
