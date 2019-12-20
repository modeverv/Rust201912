use actix_web::{fs,server,App};

fn main() {
    server::new(|| {
        App::new().handler(
            "/",
            fs::StaticFiles::new(".").unwrap(),
        )
    })
    .bind("127.0.0.1:3000")
    .expect("can not bind port 3000")
    .run()
}
