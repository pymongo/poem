use poem::{
    get, handler,
    listener::TcpListener,
    web::cookie::CookieJar,
    EndpointExt, Route, Server,
};

/*
## first time
val = no_cookie_in_req

## second time (why 123 with a extra double quote)
val = "123"
*/
#[handler]
fn a(c: &CookieJar) -> String {
    let val = match c.get("key") {
        Some(c) => c.value_str().to_string(),
        None => "no_cookie_in_req".to_string(),
    };
    println!("val = {}", val);
    c.add(poem::web::cookie::Cookie::new("key", "123"));
    val.to_string()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/", get(a))
        .with(poem::middleware::CookieJarManager::new());
    // .with(CookieSession::new(CookieConfig::default().secure(false)));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}

#[test]
fn client() {
    use reqwest::header::{SET_COOKIE, COOKIE};
    let resp = reqwest::blocking::get("http://localhost:3000").unwrap();
    let cookie = resp.headers()[SET_COOKIE].clone();
    reqwest::blocking::ClientBuilder::new().build().unwrap().get("http://localhost:3000").header(COOKIE, cookie).send().unwrap();
}
