use opendal::layers::LoggingLayer;
use opendal::{Operator, services};
use salvo::Request;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use vegar_core::AppResult;
use vegar_core::error::Error;
use vegar_core::settings::SETTINGS;

pub async fn upload(req: &mut Request, dest: &str) -> AppResult<String> {
    let op = init_s3_operator().await?;

    let file = req.file("file").await;
    if let Some(file) = file {
        let dest = Path::new(dest).join(file.name().unwrap_or("file"));
        let mut f = File::open(&file.path())?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("TODO: panic message");

        let guess = mime_guess::from_path(&file.path());

        let dest = normalize_slashes(dest.to_str().unwrap());

        op.write_with(dest.as_str(), buffer)
            .content_type(guess.first_or_octet_stream().essence_str())
            .await?;

        return Ok(dest);
    }

    Err(Error::Message("no such file or directory".into()))
}

pub async fn upload_bytes(data: Vec<u8>, filename: String) -> AppResult<String> {
    let op = init_s3_operator().await?;

    let guess = mime_guess::from_path(&filename);

    op.write_with(&filename, data)
        .content_type(guess.first_or_octet_stream().essence_str())
        .await?;

    Ok(filename)
}

fn normalize_slashes(path: &str) -> String {
    path.replace('\\', "/")
}

pub async fn file_url(path: String) -> String {
    let settings = &SETTINGS.read().await;
    format!("https://{}.{}/{}", settings.s3.bucket, settings.s3.endpoint, path)
}

pub async fn init_s3_operator() -> Result<Operator, opendal::Error> {
    let settings = &SETTINGS.read().await;

    let builder = services::S3::default()
        .region(&settings.s3.region)
        .access_key_id(&settings.s3.access_key_id)
        .secret_access_key(&settings.s3.secret_access_key)
        .endpoint(&settings.s3.endpoint)
        .enable_virtual_host_style()
        .bucket(&settings.s3.bucket);

    Ok(Operator::new(builder)?.layer(LoggingLayer::default()).finish())
}
