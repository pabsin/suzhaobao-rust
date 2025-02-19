use crate::dtos::wechat_dto::{
    AddBackgroundResult, ColourizeResult, GenerateLayoutPhotoRequest, ImageIdRequest,
    MattingRequest, UpdatePhotoDto,
};
use crate::models::_entities::photos;
use crate::services::upload_service::upload_bytes;
use redis::AsyncCommands;
use reqwest::multipart::{Form, Part};
use salvo::oapi::endpoint;
use salvo::oapi::extract::JsonBody;
use salvo::prelude::*;
use salvo::{Depot, Response};
use std::collections::HashMap;
use url::Url;
use vegar_core::error::Error;
use vegar_core::settings::SETTINGS;
use vegar_core::{bytes_to_base64, from_base64, AppResult, AppState};

#[endpoint]
pub async fn colourize(
    params: JsonBody<ImageIdRequest>,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let req = params.into_inner();
    let photo = photos::Model::find_by_id(&state.conn, &req.image_id).await?;
    let mut redis = state
        .redis
        .get_multiplexed_tokio_connection()
        .await
        .expect("Could not get redis connection");

    let image: Vec<u8> = redis.get(&photo.id).await.unwrap();
    let settings = SETTINGS.read().await;
    let url = Url::parse(&settings.id_photo_setting.colorize_uri)?.join("/colourizeImg")?;
    let client = reqwest::Client::new();
    let mut payload = HashMap::new();
    payload.insert("base64_image", bytes_to_base64(image));

    let resp = client.post(url).json(&payload).send().await?;

    let resp = if resp.status().is_success() {
        resp.json::<ColourizeResult>().await?
    } else {
        return Err(Error::Message("系统繁忙，请稍后再试".to_string()));
    };

    if resp.status != 2 {
        return Err(Error::Message("系统繁忙，请稍后再试".to_string()));
    }

    let buffer = from_base64(resp.processed_image.to_owned());
    let result = upload_bytes(buffer, photo.clone().file_name).await?;

    let dto = UpdatePhotoDto {
        name: Option::from("老照片上色".to_string()),
        file_path: Option::from(result),
        colorize_key: None,
        width: None,
        height: None,
    };

    let photo = photos::Model::update_photo(photo, &state.conn, dto).await?;

    res.render(photo.file_path);

    Ok(())
}

#[endpoint]
pub async fn generate_layout_photo(
    params: JsonBody<GenerateLayoutPhotoRequest>,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let req = params.into_inner();
    let photo = photos::Model::find_by_id(&state.conn, &req.image_id).await?;

    let mut redis = state
        .redis
        .get_multiplexed_tokio_connection()
        .await
        .expect("Could not get redis connection");

    let image: Vec<u8> = redis.get(&photo.id).await.unwrap();

    let settings = SETTINGS.read().await;
    let url = Url::parse(&settings.id_photo_setting.zjz_uri)?.join("/generate_layout_photos")?;
    let client = reqwest::Client::new();
    let mut form: Form =
        Form::new().part("input_image", Part::bytes(image).file_name(photo.file_name.clone()));

    if req.width != "" {
        form = form.text("width", req.width)
    }
    if req.height != "" {
        form = form.text("width", req.height)
    }
    if req.kb != "" {
        form = form.text("width", req.kb)
    }
    if req.dpi != "" {
        form = form.text("width", req.dpi)
    }

    let resp = client.post(url).multipart(form).send().await?;

    let resp = if resp.status().is_success() {
        resp.json::<AddBackgroundResult>().await?
    } else {
        return Err(Error::Message("系统繁忙，请稍后再试".to_string()));
    };

    if resp.status != true {
        return Err(Error::Message("未检测到人脸或多人脸".to_string()));
    }

    let buffer = from_base64(resp.image_base64.to_owned());
    let result = upload_bytes(buffer, photo.clone().file_name).await?;

    let dto = UpdatePhotoDto {
        name: Option::from("六寸排版照".to_string()),
        file_path: Option::from(result),
        colorize_key: None,
        width: None,
        height: None,
    };

    let photo = photos::Model::update_photo(photo, &state.conn, dto).await?;

    res.render(photo.file_path);

    Ok(())
}

#[endpoint]
pub async fn matting(
    params: JsonBody<MattingRequest>,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let req = params.into_inner();
    let photo = photos::Model::find_by_id(&state.conn, &req.image_id).await?;

    let mut redis = state
        .redis
        .get_multiplexed_tokio_connection()
        .await
        .expect("Could not get redis connection");

    let image: Vec<u8> = redis.get(&photo.id).await.unwrap();

    let settings = SETTINGS.read().await;
    let url = Url::parse(&settings.id_photo_setting.zjz_uri)?.join("/human_matting")?;
    let client = reqwest::Client::new();
    let mut form: Form = Form::new()
        .text("human_matting_model", settings.clone().id_photo_setting.human_matting_model)
        .part("input_image", Part::bytes(image).file_name(photo.file_name.clone()));

    if req.dpi != "" {
        form = form.text("width", req.dpi)
    }

    let resp = client.post(url).multipart(form).send().await?;

    let resp = if resp.status().is_success() {
        resp.json::<AddBackgroundResult>().await?
    } else {
        return Err(Error::Message("系统繁忙，请稍后再试".to_string()));
    };

    if resp.status != true {
        return Err(Error::Message("未检测到人脸或多人脸".to_string()));
    }

    let buffer = from_base64(resp.image_base64.to_owned());
    let result = upload_bytes(buffer, photo.clone().file_name).await?;

    let dto = UpdatePhotoDto {
        name: Option::from("图片抠图".to_string()),
        file_path: Option::from(result),
        colorize_key: None,
        width: None,
        height: None,
    };

    let photo = photos::Model::update_photo(photo, &state.conn, dto).await?;

    res.render(photo.file_path);

    Ok(())
}

#[endpoint]
pub async fn cartoon(
    params: JsonBody<MattingRequest>,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let req = params.into_inner();
    let photo = photos::Model::find_by_id(&state.conn, &req.image_id).await?;

    let mut redis = state
        .redis
        .get_multiplexed_tokio_connection()
        .await
        .expect("Could not get redis connection");

    let image: Vec<u8> = redis.get(&photo.id).await.unwrap();

    let settings = SETTINGS.read().await;
    let url = Url::parse(&settings.id_photo_setting.cartoon_uri)?.join("/cartoon")?;
    let client = reqwest::Client::new();

    let mut payload = HashMap::new();
    payload.insert("image", bytes_to_base64(image));
    let resp = client.post(url).json(&payload).send().await?;

    let resp = if resp.status().is_success() {
        resp.json::<AddBackgroundResult>().await?
    } else {
        return Err(Error::Message("系统繁忙，请稍后再试".to_string()));
    };

    if resp.status != true {
        return Err(Error::Message("未检测到人脸或多人脸".to_string()));
    }

    let buffer = from_base64(resp.image_base64.to_owned());
    let result = upload_bytes(buffer, photo.clone().file_name).await?;

    let dto = UpdatePhotoDto {
        name: Option::from("图片抠图".to_string()),
        file_path: Option::from(result),
        colorize_key: None,
        width: None,
        height: None,
    };

    let photo = photos::Model::update_photo(photo, &state.conn, dto).await?;

    res.render(photo.file_path);

    Ok(())
}
