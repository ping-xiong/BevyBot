use actix_web::{HttpResponse, http::StatusCode};
use serde::Serialize;

use crate::HttpResult;

#[derive(Debug, Serialize)]
pub struct Res<T> where T:Serialize {
    pub code: i32,
    pub msg: String,
    pub data: T
}

#[derive(Debug, Serialize)]
pub struct ListRes<T> where T:Serialize {
    pub list: T,
    pub total: u64
}

/// 成功返回
pub fn success_ret<T>(data: T) -> HttpResult where T:Serialize {
    Ok(
        HttpResponse::Ok().json(Res {
            code: 0,
            msg: "".to_string(),
            data
        })
    )
}


/// 失败返回
pub fn fail_ret(msg: &str) -> HttpResult {
    Ok(
        HttpResponse::Ok()
        .json(Res {
            code: -1,
            msg: msg.to_string(),
            data: ""
        })
    )
}


/// 未登录
pub fn not_login() -> HttpResult {
    Ok(
        HttpResponse::new(StatusCode::UNAUTHORIZED)
    )
}