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
