use crate::dtos::user_dto::{CurrentResponse, RegisterInput, UpdateUserInput};
use crate::models::_entities::users;
use salvo::Writer;
use salvo::http::StatusError;
use salvo::oapi::endpoint;
use salvo::oapi::extract::JsonBody;
use salvo::prelude::Json;
use salvo::{Depot, Response};
use vegar_core::{AppResult, AppState};

/// 当前登录用户信息
#[endpoint]
pub async fn me(depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let user = depot.obtain::<users::Model>().map_err(|_| StatusError::internal_server_error())?;

    res.render(Json(CurrentResponse::new(user)));

    Ok(())
}

#[endpoint]
pub async fn update(
    params: JsonBody<UpdateUserInput>,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;

    let user = depot.obtain::<users::Model>().map_err(|_| StatusError::internal_server_error())?;

    let item = users::Model::update_user(user.clone(), &state.conn, params.into_inner()).await?;
    res.render(Json(item));

    // res.render(Json(user));

    Ok(())
}

/// 注册
#[endpoint]
pub async fn register(
    params: JsonBody<RegisterInput>,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;

    let user = users::Model::create_with_password(&state.conn, &params.into_inner()).await?;
    res.render(Json(user));

    Ok(())
}
