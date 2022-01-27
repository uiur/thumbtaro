use std::{env::{temp_dir, self}, fs::{File, create_dir_all}, path::Path};

use actix_files::NamedFile;
use actix_web::{get, HttpServer, App, Result, web, error};
use cloud_storage;

use std::io::Write;
use dotenv::dotenv;

struct CloudStoragePath {
    pub path: String
}

impl CloudStoragePath {
    pub fn from(key: &str) -> CloudStoragePath {
        let key_prefix = env::var("THUMBTARO_KEY_PREFIX").unwrap_or(String::from(""));
        CloudStoragePath { path: format!("{}{}", key_prefix, key) }
    }

    fn original_path(&self) -> String {
        self.path.clone()
    }

    fn thumbnail_path(&self, width: u32, height: u32) -> String {
        let path = std::path::Path::new(&(self.path));
        let dir_path = path.parent().unwrap();
        let filename = path.file_name().unwrap().to_str().unwrap();
        format!("{}/{}x{}_{}", dir_path.display(), width, height, filename)
    }
}

#[get("/orig/{key:.*}")]
async fn original(path: web::Path::<(String,)>) -> Result<NamedFile> {
    let key = path.into_inner().0;
    let file_path = download_original(&key).await?;

    let file = File::open(&file_path)?;
    Ok(NamedFile::from_file(file, &file_path)?)
}

async fn try_download_thumbnail(key: &str, width: u32, height: u32) -> Option<String> {
    let cloud_storage_path = CloudStoragePath::from(&key);
    let client = cloud_storage::Client::default();
    let thumbnail_path = cloud_storage_path.thumbnail_path(width, height);


    let bucket = env::var("THUMBTARO_BUCKET").expect("THUMBTARO_BUCKET must be specified");
    if let Ok(bytes) = client.object().download(&bucket, &thumbnail_path).await {
       return match save_file(&thumbnail_path, bytes) {
           Ok(s) => Some(s),
           Err(_) => None
       }
    }

    None
}

#[get("/thumb/{width}x{height}/{key:.*}")]
async fn thumb(path: web::Path::<(u32, u32, String)>) -> Result<NamedFile> {
    let (width, height, key) = path.into_inner();

    if let Some(thumbnail_path) = try_download_thumbnail(&key, width, height).await {
        return Ok(NamedFile::open(&thumbnail_path)?)
    }

    let file_path = download_original(&key).await?;

    let img = image::open(&file_path).map_err (|e| error::ErrorInternalServerError(e) )?;
    let img = img.thumbnail(width, height);
    img.save(&file_path).map_err(|e| error::ErrorInternalServerError(e))?;

    let file = File::open(&file_path)?;
    Ok(NamedFile::from_file(file, &file_path)?)
}

fn save_file(path: &str, bytes: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    let file_path = format!("{}/{}", temp_dir().display(), path);
    let dir_path = Path::new(&file_path).parent().unwrap();
    create_dir_all(dir_path)?;

    let mut file = File::create(&file_path)?;
    file.write_all(&bytes)?;

    Ok(file_path)
}

async fn download_original(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cloud_storage_path = CloudStoragePath::from(key);

    let bucket = env::var("THUMBTARO_BUCKET").expect("THUMBTARO_BUCKET must be specified");
    let client = cloud_storage::Client::default();
    let bytes = client.object().download(&bucket, &cloud_storage_path.original_path()).await?;
    save_file(key, bytes)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

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
