// backend/src/main.rs V 1.3 (路徑修正)
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use tera::Tera;

mod db;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 伺服器正在 http://{}:{} 啟動","192.168.2.152", 8888);

    HttpServer::new(move || {
        // 初始化 Tera 樣板引擎，並指定樣板檔案的路徑
        // 路徑是相對於執行 `cargo run` 的 `backend` 目錄
        let tera = match Tera::new("../frontend/templates/**/*") { // V 1.3 修改路徑
            Ok(t) => t,
            Err(e) => {
                println!("樣板引擎初始化錯誤(Tera): {:?}", e); // 更明確的錯誤訊息
                std::process::exit(1);
            }
        };

        App::new()
            .app_data(web::Data::new(tera.clone()))
            .configure(routes::configure_routes)
            .service(
                Files::new("/static", "../frontend/static") // V 1.3 修改路徑
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind(("192.168.2.152", 8888))?
    .run()
    .await
}