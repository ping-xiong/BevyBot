use actix_web::{Scope, web};

pub fn client() -> Scope {
    web::scope("client")
}