use data_encoding::BASE64URL;
use hmac::{Hmac, Mac};
use opendal::layers::LoggingLayer;
use opendal::{Operator, services};
use salvo::Request;
use sha1::Sha1;
use sha1::digest::KeyInit;
use std::fs::File;
use std::io::Read;
use vegar_core::AppResult;
use vegar_core::error::Error;
use vegar_core::settings::SETTINGS;

pub async fn upload(req: &mut Request) -> AppResult<String> {
    let settings = &SETTINGS.read().await;

    let builder = services::S3::default()
        .region(&settings.s3.region)
        .access_key_id(&settings.s3.access_key_id)
        .secret_access_key(&settings.s3.secret_access_key)
        .endpoint(&settings.s3.endpoint)
        .enable_virtual_host_style()
        .bucket(&settings.s3.bucket);

    let op = Operator::new(builder)?.layer(LoggingLayer::default()).finish();

    let file = req.file("file").await;
    if let Some(file) = file {
        let dest = file.name().unwrap_or("file");
        let mut f = File::open(&file.path())?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("TODO: panic message");

        let guess = mime_guess::from_path(&file.path());

        op.write_with(&dest, buffer)
            .content_type(guess.first_or_octet_stream().essence_str())
            .await?;

        let url = format!("https://{}.{}/{}", &settings.s3.bucket, &settings.s3.endpoint, &dest);

        return Ok(url);
    }

    Err(Error::Message("no such file or directory".into()))
}

pub async fn upload_bytes(data: Vec<u8>, filename: String) -> AppResult<String> {
    let settings = &SETTINGS.read().await;

    let builder = services::S3::default()
        .region(&settings.s3.region)
        .access_key_id(&settings.s3.access_key_id)
        .secret_access_key(&settings.s3.secret_access_key)
        .endpoint(&settings.s3.endpoint)
        .enable_virtual_host_style()
        .bucket(&settings.s3.bucket);

    let op = Operator::new(builder)?.layer(LoggingLayer::default()).finish();

    let guess = mime_guess::from_path(&filename);
    op.write_with(&filename, data)
        .content_type(guess.first_or_octet_stream().essence_str())
        .await?;

    let url = format!("https://{}.{}/{}", &settings.s3.bucket, &settings.s3.endpoint, &filename);

    Ok(url)
}

type HmacSha1 = Hmac<Sha1>;

pub fn sign(path: String, secret: &str) -> String {
    let mut mac =
        HmacSha1::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");

    mac.update(path.as_bytes());

    let result = mac.finalize();
    let hash = BASE64URL.encode(result.into_bytes().as_slice());

    format!("{}/{}", hash, path)
}
