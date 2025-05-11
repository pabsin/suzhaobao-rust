use crate::dtos::user_dto::{CurrentResponse, RegisterInput, UpdateUserInput};
use crate::models::_entities::users;
use salvo::http::StatusError;
use salvo::prelude::Json;
use salvo::{Depot, Response};
use salvo::{Request, handler};
use vegar_core::{AppResult, AppState};

#[handler]
pub async fn me(depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let user = depot.obtain::<users::Model>().map_err(|_| StatusError::internal_server_error())?;

    res.render(Json(CurrentResponse::new(user)));

    Ok(())
}

#[handler]
pub async fn update(req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;

    let user = depot.obtain::<users::Model>().map_err(|_| StatusError::internal_server_error())?;
    let params = req.parse_json::<UpdateUserInput>().await?;

    let item = users::Model::update_user(user.clone(), &state.conn, params).await?;
    res.render(Json(item));

    Ok(())
}

#[handler]
pub async fn register(req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let params = req.parse_json::<RegisterInput>().await?;
    let user = users::Model::create_with_password(&state.conn, &params).await?;

    res.render(Json(user.id));

    Ok(())
}
