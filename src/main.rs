// #[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::serde::Deserialize;
use rocket::{get, launch, routes};

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
    // ルーティング
    let rocket = rocket::build()
        .mount("/", routes![index]) // ルートディレクトリへのマウント
        .mount("/sample", routes![sample]) // sampleディレクトリへのマウント
        .attach(Template::fairing());
    
    // コンフィグまわり
    let figment = rocket.figment();
    #[derive(Deserialize, Debug)]
    #[serde(crate = "rocket::serde")]
    struct Config {
        port: u16,
    }

    // コンフィグ設定とその簡単な確認(printlnは余計かも)
    let config: Config = figment.extract().expect("config");
    println!("{:?}", config);

    rocket
}