#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { field: "value" })
}

#[get("/echo")] // ルーティング先のディレクトリ構造(/sampleにマウントしているので、パスはsample/echoとなる)
fn sample() -> &'static str { // リクエストハンドラー
    "Echo Sample"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index]) // ルートディレクトリへのマウント
        .mount("/sample", routes![sample]) // sampleディレクトリへのマウント
        .attach(Template::fairing())
}