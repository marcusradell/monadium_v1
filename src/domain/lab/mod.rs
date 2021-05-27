use actix_web::{
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};

#[derive(Clone, Debug)]
pub struct Lab {
    pub foo: String,
}

impl Lab {
    fn controller(self, req: HttpRequest) -> impl Responder {
        println!("I got access to foo: {:?}", self.foo);

        HttpResponse::Ok().finish()
    }

    pub fn config(self, cfg: &mut ServiceConfig) {
        cfg.service(web::scope("lab").route(
            "foo",
            web::get().to(move |req| {
                self.clone().controller(req);
                HttpResponse::Ok().finish()
            }),
        ));
    }
}
