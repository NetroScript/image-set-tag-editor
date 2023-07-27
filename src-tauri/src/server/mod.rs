
use std::sync::Mutex;

use tauri::AppHandle;
use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Result, middleware};
use std::path::PathBuf;
use super::PORT;
use super::SERVED_DIR;

struct TauriAppState {
    app: Mutex<AppHandle>,
}

#[actix_web::main]
pub async fn init(app: AppHandle) -> std::io::Result<()> {
    let tauri_app = web::Data::new(TauriAppState {
        app: Mutex::new(app),
    });

    // Create server, bind it to any available port
    let server = HttpServer::new(move || {
        App::new()
            .app_data(tauri_app.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/{filename:.*}").route(web::get().to(serve_file)))
    })
    .bind("127.0.0.1:0")?;

    // Get the actual bound port and store it
    if let Some(local_addr) = server.addrs().get(0) {
        let mut port_write = PORT.write().unwrap();
        *port_write = Some(local_addr.port());
    }

    // Print out the port number and the currently served absolute path to the console
    println!(
        "Serving files from {} on port {}",
        SERVED_DIR.read().unwrap(),
        PORT.read().unwrap().unwrap()
    );

    // Await the server
    server.run().await
}


async fn serve_file(path: web::Path<(String,)>) -> Result<NamedFile> {
    let directory = SERVED_DIR.read().unwrap();
    let file_path = PathBuf::from(&*directory).join(&path.0);

    // Prevent directory traversal attacks
    if !file_path.starts_with(&*directory) {
        return Err(actix_web::error::ErrorNotFound("Not Found"));
    }

    Ok(NamedFile::open(file_path)?)
}
