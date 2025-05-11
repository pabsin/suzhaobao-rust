use crate::models::_entities::users;
use salvo::http::{StatusCode, StatusError};
use salvo::jwt_auth::JwtAuthState;
use salvo::prelude::JwtAuthDepotExt;
use salvo::{Depot, FlowCtrl, Request, Response, handler};
use vegar_core::JwtClaims;
use vegar_core::{AppResult, AppState};

#[handler]
pub async fn must_login(
    _req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> AppResult<()> {
    let state = depot.obtain::<AppState>().map_err(|_| StatusError::internal_server_error())?;

    match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {
            let data = depot.jwt_auth_data::<JwtClaims>().unwrap();
            let user = users::Model::find_by_username(&state.conn, &data.claims.username).await?;
            depot.inject(user);
        }
        JwtAuthState::Unauthorized => {
            ctrl.skip_rest();
            res.status_code(StatusCode::UNAUTHORIZED);
        }
        JwtAuthState::Forbidden => {
            ctrl.skip_rest();
            res.status_code(StatusCode::FORBIDDEN);
        }
    }

    Ok(())
}
