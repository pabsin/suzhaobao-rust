use crate::dtos::wechat_dto::{
    AddBackgroundResult, CreateIdPhotoRequest, CreateIdPhotoResult, IdPhotoResponse,
    ImageIdRequest, PhotoListRequest, UpdateIdPhotoRequest, UpdatePhotoDto,
};
use crate::models::_entities::{photo_items, photos, users};
use crate::services::upload_service::{file_url, upload_bytes};
use cuid::cuid2;
use redis::AsyncCommands;
use reqwest::multipart::{Form, Part};
use salvo::prelude::*;
use salvo::{Depot, Response};
use std::ops::Deref;
use url::Url;
use vegar_core::error::Error;
use vegar_core::settings::SETTINGS;
use vegar_core::{AppResult, AppState, from_base64};

#[handler]
pub async fn photo_list(req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let user = depot.obtain::<users::Model>().map_err(|_| StatusError::internal_server_error())?;
    let params = req.parse_queries::<PhotoListRequest>()?;

    let result = photos::Model::find_items(&state.conn, &params, user.id.as_str()).await?;
    res.render(Json(result));

    Ok(())
}

#[handler]
pub async fn create_id_photo(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let params = req.parse_json::<CreateIdPhotoRequest>().await?;

    let mut redis = state
        .redis
        .get_multiplexed_tokio_connection()
        .await
        .expect("Could not get redis connection");
    let image: Vec<u8> = redis.get(&params.image_id).await.unwrap();

    let item = photo_items::Model::find_by_id(&state.conn, params.item_id.deref()).await?;

    let photo = photos::Model::find_by_id(&state.conn, &params.image_id).await?;

    let settings = SETTINGS.read().await;
    let url = Url::parse(&settings.id_photo_setting.zjz_uri)?.join("/idphoto")?;
    let client = reqwest::Client::new();
    let mut form: Form = Form::new()
        .text("height", item.height_px.to_string())
        .text("dpi", item.dpi.to_string())
        .text("width", item.width_px.to_string())
        .text("face_alignment", "true")
        .text("hd", "true")
        .text("human_matting_model", settings.clone().id_photo_setting.human_matting_model)
        .part("input_image", Part::bytes(image).file_name(photo.file_name.clone()));

    if params.is_beauty_on == 1 {
        form = form
            .text("brightness_strength", "1")
            .text("contrast_strength", "1")
            .text("sharpen_strength", "1")
            .text("saturation_strength", "1")
    }

    let resp = client.post(url).multipart(form).send().await?;

    let resp = if resp.status().is_success() {
        resp.json::<CreateIdPhotoResult>().await?
    } else {
        return Err(Error::Message("系统繁忙，请稍后再试".to_string()));
    };

    if resp.status != true {
        return Err(Error::Message("未检测到人脸或多人脸".to_string()));
    }

    let buffer = from_base64(resp.image_base64_standard.to_owned());

    let dto = UpdatePhotoDto {
        name: Option::from(item.name),
        file_path: None,
        colorize_key: None,
        width: Option::from(item.width_px),
        height: Option::from(item.height_px),
    };

    let photo = photos::Model::update_photo(photo, &state.conn, dto).await?;

    let _: () = redis::pipe()
        .atomic()
        .set(&photo.id, &buffer)
        .ignore()
        .expire(&photo.id, 600)
        .query_async(&mut redis)
        .await
        .unwrap();

    res.render(Json(IdPhotoResponse {
        image_id: photo.id,
        image_base64: resp.image_base64_standard,
    }));

    Ok(())
}

#[handler]
pub async fn update_id_photo(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let params = req.parse_json::<UpdateIdPhotoRequest>().await?;
    let mut redis = state
        .redis
        .get_multiplexed_tokio_connection()
        .await
        .expect("Could not get redis connection");

    let photo = photos::Model::find_by_id(&state.conn, &params.image_id).await?;

    let image: Vec<u8> = redis.get(&photo.id).await.unwrap();

    let url =
        Url::parse(&SETTINGS.read().await.id_photo_setting.zjz_uri)?.join("/add_background")?;
    let client = reqwest::Client::new();
    let form = Form::new()
        .text("render", params.render.to_string())
        .text("color", params.color)
        .part("input_image", Part::bytes(image).file_name(photo.clone().file_name));

    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .expect("error")
        .json::<AddBackgroundResult>()
        .await?;

    if resp.status != true {
        return Err(Error::Message("未检测到人脸或多人脸".to_string()));
    }

    let buffer = from_base64(resp.image_base64.to_owned());
    let colorize_key = cuid2();

    let dto = UpdatePhotoDto {
        colorize_key: Option::from(colorize_key.to_owned()),
        ..Default::default()
    };
    let photo = photos::Model::update_photo(photo, &state.conn, dto).await?;

    let _: () = redis::pipe()
        .atomic()
        .set(&photo.colorize_key, &buffer)
        .ignore()
        .expire(&photo.colorize_key, 600)
        .query_async(&mut redis)
        .await
        .unwrap();

    res.render(Json(IdPhotoResponse { image_id: photo.id, image_base64: resp.image_base64 }));

    Ok(())
}

#[handler]
pub async fn download_photo(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let params = req.parse_json::<ImageIdRequest>().await?;
    let photo = photos::Model::find_by_id(&state.conn, &params.image_id).await?;
    let mut redis = state
        .redis
        .get_multiplexed_tokio_connection()
        .await
        .expect("Could not get redis connection");

    let image: Vec<u8> = redis.get(&photo.colorize_key).await.unwrap();

    let result = upload_bytes(image, photo.clone().file_name).await?;

    let dto = UpdatePhotoDto { file_path: Option::from(result.to_owned()), ..Default::default() };

    photos::Model::update_photo(photo, &state.conn, dto).await?;

    res.render(file_url(result).await);

    Ok(())
}

#[handler]
pub async fn delete_photo(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let user = depot.obtain::<users::Model>().map_err(|_| StatusError::internal_server_error())?;
    let params = req.parse_json::<ImageIdRequest>().await?;

    photos::Model::delete_user_photo_by_id(&state.conn, params.image_id.as_str(), user.id.as_str())
        .await?;

    res.render("");

    Ok(())
}

#[handler]
pub async fn photo_detail(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let id = req.query::<String>("id").expect("id must be set");

    let result = photos::Model::find_by_id(&state.conn, &id).await?;

    res.render(Json(result));

    Ok(())
}
