#[macro_use] extern crate rocket;

#[get("/")] // ルーティング先のディレクトリ構造
fn index() -> &'static str { // リクエストハンドラー
    "Hello, world!" // TODO: HTMLを呼び出すには？
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
