use crate::dtos::wechat_dto::ItemListRequest;
use crate::models::_entities::photo_items;
use salvo::oapi::endpoint;
use salvo::prelude::*;
use salvo::{Depot, Response};
use vegar_core::{AppResult, AppState};

#[endpoint]
pub async fn photo_item_list(
    params: ItemListRequest,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let result = photo_items::Model::find_items(&state.conn, &params).await?;
    res.render(Json(result));

    Ok(())
}

#[endpoint]
pub async fn photo_item(req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;
    let id = req.query::<String>("id").expect("id must be set");

    let result = photo_items::Model::find_by_id(&state.conn, &id).await?;
    res.render(Json(result));

    Ok(())
}
