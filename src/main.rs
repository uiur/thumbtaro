use std::{env::temp_dir, fs::{File, create_dir_all}, path::Path};

use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder, get, HttpServer, App, Result, web, error};
use cloud_storage;
use std::io::Write;

#[get("/orig/{key:.*}")]
async fn original(path: web::Path::<(String,)>) -> Result<NamedFile> {
    let key = path.into_inner().0;
    let file_path = download_original(&key).await?;

    let file = File::open(&file_path)?;
    Ok(NamedFile::from_file(file, &file_path)?)
}

#[get("/thumb/{width}x{height}/{key:.*}")]
async fn thumb(path: web::Path::<(u32, u32, String)>) -> Result<NamedFile> {
    let (width, height, key) = path.into_inner();
    let file_path = download_original(&key).await?;

    let img = image::open(&file_path).map_err (|e| error::ErrorInternalServerError(e) )?;
    let img = img.thumbnail(width, height);
    img.save(&file_path).map_err(|e| error::ErrorInternalServerError(e))?;


    let file = File::open(&file_path)?;
    Ok(NamedFile::from_file(file, &file_path)?)
}

async fn download_original(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = format!("uploads/{}", key);

    let client = cloud_storage::Client::default();
    let bytes = client.object().download("autoreserve", &key).await?;

    let file_path = format!("{}/{}", temp_dir().display(), key);
    let dir_path = Path::new(&file_path).parent().unwrap();
    create_dir_all(dir_path)?;

    let mut file = File::create(&file_path)?;
    file.write_all(&bytes)?;

    Ok(file_path)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    HttpServer::new(|| {
        App::new()
            .service(original)
            .service(thumb)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
