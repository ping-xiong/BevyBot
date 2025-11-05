use actix_web::post;

use crate::{util::res::success_ret, HttpResult};


#[post("login")]
pub async fn login() -> HttpResult {

    success_ret("")
}