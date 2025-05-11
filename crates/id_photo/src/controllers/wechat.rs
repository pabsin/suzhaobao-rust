use crate::dtos::user_dto::RegisterInput;
use crate::dtos::wechat_dto::{
    CreatePhotoDto, GetVideoUnitResponse, MiniAppLoginRequest, UpdateUsernameRequest,
};
use crate::models::_entities::{app_settings, photos, users};
use crate::services::upload_service;
use cuid::cuid2;
use labrador::{SimpleStorage, WechatMaClient};
use salvo::http::cookie::Cookie;
use salvo::http::cookie::time::{Duration, OffsetDateTime};
use salvo::prelude::*;
use salvo::{Depot, Response};
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use vegar_core::settings::SETTINGS;
use vegar_core::{AppResult, AppState};

#[handler]
pub async fn miniapp_login(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let params = req.parse_json::<MiniAppLoginRequest>().await?;
    let settings = app_settings::Model::find_by_app_id(&state.conn, &params.app_id).await?;

    let client = WechatMaClient::<SimpleStorage>::new(settings.app_id, settings.app_secret);
    let result = client.code_session().jscode_2_session(&params.code).await.expect("dsdsds");

    let mut user_model = users::Model::find_by_open_id(&state.conn, result.openid.deref()).await;
    if user_model.is_err() {
        let input = RegisterInput {
            email: cuid2() + "@vegar.cn",
            username: cuid2(),
            password: cuid2(),
            name: "陌生人".to_owned(),
            open_id: Option::from(result.openid.clone()),
        };
        user_model = Ok(users::Model::create_with_password(&state.conn, &input)
            .await
            .expect("can not create user"));
    }

    let user = user_model?;
    let settings = &SETTINGS.read().await;
    let exp = OffsetDateTime::now_utc() + Duration::days(14);
    let token = user.generate_jwt(&settings.auth.jwt_secret, exp.unix_timestamp())?;

    res.add_cookie(Cookie::new("token", token.clone())).render(token);

    Ok(())
}

#[handler]
pub async fn update_user_avatar(
    req: &mut Request,
    depot: &mut Depot,
    _res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let user = depot.obtain::<users::Model>().map_err(|_| StatusError::internal_server_error())?;

    let url = upload_service::upload(req, "id_photo").await?;

    users::Model::update_user_avatar(user.clone(), &state.conn, url).await?;

    Ok(())
}

#[handler]
pub async fn update_user_nickname(
    req: &mut Request,
    depot: &mut Depot,
    _res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let user = depot.obtain::<users::Model>().map_err(|_| StatusError::internal_server_error())?;
    let params = req.parse_json::<UpdateUsernameRequest>().await?;

    users::Model::update_user_name(user.clone(), &state.conn, params.name).await?;

    Ok(())
}

#[handler]
pub async fn upload(req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let user = depot.obtain::<users::Model>().map_err(|_| StatusError::internal_server_error())?;

    let file = req.file("file").await;
    if let Some(file) = file {
        let mut f = File::open(&file.path())?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("TODO: panic message");

        let dto = CreatePhotoDto {
            user_id: user.id.clone(),
            name: "".to_string(),
            file_path: "".to_string(),
            processing_key: "".to_string(),
            width: 0,
            file_name: file.name().unwrap_or("img.jpg").parse().unwrap(),
            height: 0,
        };

        let photo = photos::Model::create_photo(&state.conn, &dto).await?;

        let mut con = state
            .redis
            .get_multiplexed_tokio_connection()
            .await
            .expect("Could not get redis connection");

        let _: () = redis::pipe()
            .atomic()
            .set(&photo.id, &buffer)
            .ignore()
            .expire(&photo.id, 600)
            .query_async(&mut con)
            .await
            .unwrap();

        res.render(&photo.id);
    }

    Ok(())
}

#[handler]
pub async fn get_video_unit(
    _req: &mut Request,
    _depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    res.render(Json(GetVideoUnitResponse { video_unit_id: "".to_string(), download_hd: 0 }));

    Ok(())
}
