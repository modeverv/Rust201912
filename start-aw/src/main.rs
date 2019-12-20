use actix_web::{server, App, Error, FromRequest, HttpRequest, Path, Responder, State,fs};
use serde_derive::*;

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}
/*
fn hello(req: &HttpRequest) -> Result<String, Error> {
    //let to = req.match_info().get("name").unwrap_or("World");
    let to = Path::<HelloPath>::extract(req)?;
    Ok(format!("Hello {}!", &to.name))
}
*/
fn hello_name(to: Path<HelloPath>) -> impl Responder {
    format!("Hello {}!", &to.name)
}

// アプリケーション情報を保持するデータ型
struct MyApp {
    server_name: String,
}

fn hello_with_state(app: State<MyApp>) -> Result<String, Error> {
    Ok(format!("Hello from {}!", &app.server_name))
}

fn main() {
    let port = 3000;
    server::new(|| {
        //        App::new()
        App::with_state(MyApp {
            server_name: "server with state".into(),
        })
        //        .resource("/", |r| r.f(hello))
        //        .resource("/{name}", |r| r.f(hello))
        .resource("/info", |r| r.with(hello_with_state))
        .resource("/", |r| r.with(hello_name))
        .resource("/{name}", |r| r.with(hello_name))
    })
    .bind(format!("127.0.0.1:{}", port))
    .expect(&format!("Can not bind to port {}", port))
    .run();
}
