use actix_web::guard::GuardContext;

use crate::config;

pub fn verify_api_key(ctx: &GuardContext) -> bool {
    let auth_header = ctx.head().headers().get("x-api-key");
    let api_key = config::get_api_key();
    match auth_header {
        Some(auth) => {
            if auth == &api_key {
                true
            } else {
                false
            }
        }
        None => false,
    }
}
