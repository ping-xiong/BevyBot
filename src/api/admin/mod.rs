use actix_web::{Scope, web};
mod login;
mod middleware;

pub fn admin() -> Scope {
    web::scope("admin")
        .service(login::login)
        .service(
            // 把需要权限验证的接口放在一起
            web::scope("manager")
            .wrap(middleware::CheckLogin)
        )
}
