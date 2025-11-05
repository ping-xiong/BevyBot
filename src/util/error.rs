use derive_more::{Display, Error};
use actix_web::http::header::TryIntoHeaderValue;
use log::error;
use migration::Write;

use super::res::Res;

pub type Result<T> = std::result::Result<T, MyError>;

#[derive(Debug, Display, Error)]
pub struct MyError {
    err: anyhow::Error
}

impl actix_web::error::ResponseError for MyError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::OK
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        error!("{:?}", self.err);
        let mut res = actix_web::HttpResponse::new(self.status_code());
        let mut buf = actix_web::web::BytesMut::new();
        let body = Res {
            code: -1,
            msg: "服务器内部发生错误".into(),
            data: "".to_string()
        };
        let body_str = serde_json::to_string(&body).unwrap_or("".to_string());
        let _ = buf.write_str(&body_str);

        let mime = mime::TEXT_PLAIN_UTF_8.try_into_value().unwrap();
        res.headers_mut().insert(actix_web::http::header::CONTENT_TYPE, mime);

        res.set_body(actix_web::body::BoxBody::new(buf))
    }
}

impl From<anyhow::Error> for MyError {
    fn from(err: anyhow::Error) -> Self {
        MyError { err }
    }
}

impl From<actix_web::Error> for MyError {
    fn from(err: actix_web::Error) -> Self {
        MyError { err: anyhow::anyhow!(err.to_string())}
    }
}

impl From<sea_orm::DbErr> for MyError {
    fn from(err: sea_orm::DbErr) -> Self {
        MyError { err: anyhow::anyhow!(err.to_string())}
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError { err: anyhow::anyhow!(err.to_string())}
    }
}

impl From<std::env::VarError> for MyError {
    fn from(err: std::env::VarError) -> Self {
        MyError { err: anyhow::anyhow!(err.to_string())}
    }
}

impl From<redis::RedisError> for MyError {
    fn from(err: redis::RedisError) -> Self {
        MyError { err: anyhow::anyhow!(err.to_string())}
    }
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> Self {
        MyError { err: anyhow::anyhow!(err.to_string())}
    }
}