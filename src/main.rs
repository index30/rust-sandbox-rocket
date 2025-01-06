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
        .attach(Template::fairing()); // テンプレート機能の有効化
    
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

// テストコード
#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::uri;
    use rocket::routes;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn sample() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        println!("{}",uri!(super::sample)); // sampleは/sampleにマウントしているが、get内のパスを参照して、/echoと表示される
        let mut response = client.get("/sample/echo").dispatch(); //直接指定すると通るが・・・

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert!(response.headers().get_one("X-Content-Type-Options").is_some());
        assert_eq!(response.into_string().unwrap(), "Echo Sample");
    }
}