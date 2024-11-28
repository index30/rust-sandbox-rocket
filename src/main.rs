#[macro_use] extern crate rocket;

#[get("/")] // ルーティング先のディレクトリ構造
fn index() -> &'static str { // リクエストハンドラー
    "Hello, world!" // TODO: HTMLを呼び出すには？
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
}
