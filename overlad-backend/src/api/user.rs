use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use overlad_api::User;

use crate::{db::user::DbUser, util::{internal_server_error, to_row_not_found}, AppState};

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<User>, (StatusCode, String)> {
    let db_user = DbUser::get_by_id(&state.pool, user_id)
        .await
        .and_then(to_row_not_found)
        .map_err(internal_server_error)?;

    Ok(Json(User::from(db_user)))
}
