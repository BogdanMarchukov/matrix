use dotenv::dotenv;

pub fn get_port() -> u16 {
    dotenv().ok();
    let mut port = 3000;
    match std::env::var("PORT") {
        Ok(val) => match val.parse() {
            Ok(p) => port = p,
            Err(e) => println!("error: {}", e),
        },
        Err(e) => println!("error: {}", e),
    }
    port
}

pub fn get_host() -> String {
    dotenv().ok();
    let mut host = String::from("127.0.0.1");
    match std::env::var("HOST") {
        Ok(val) => host = val,
        Err(e) => println!("error: {}", e),
    };
    host
}

pub fn get_database_url() -> String {
    dotenv().ok();
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_bot_token() -> String {
    dotenv().ok();
    std::env::var("BOT_TOKEN").expect("BOT_TOKEN must be set")
}

pub fn get_jwt_sectet() -> String {
    dotenv().ok();
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn get_api_key() -> String {
    dotenv().ok();
    std::env::var("API_KEY").expect("API_KEY must be set")
}

pub fn get_node_env() -> String {
    dotenv().ok();
    std::env::var("NODE_ENV").expect("NODE_ENV must be set")
}

pub fn get_minio_access_secret_key() -> String {
    dotenv().ok();
    std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set")
}

pub fn get_minio_access_key() -> String {
    dotenv().ok();
    std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set")
}
