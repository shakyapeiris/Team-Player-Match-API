use actix_web::{
    http::StatusCode,
    post,
    web::{self, Json},
};
use ulid::Ulid;

use crate::{
    models::{Player, PlayerBody},
    utils::{AppErrorType, AppResponse, AppState},
};

#[post("/")]
async fn create(
    app_state: web::Data<AppState>,
    body: Json<PlayerBody>,
) -> Result<Json<AppResponse<Player>>, AppErrorType> {
    let ulid = Ulid::new();
    let new_player = Player {
        PK: (&body.team).to_string(),
        SK: format!("P#{}", ulid.to_string()),
        fname: (&body.fname).to_string(),
        lname: (&body.lname).to_string(),
        photo: (&body.photo).to_string(),
    };

    let result = app_state.ddb.put_player(new_player.clone()).await;

    match result {
        Ok(_) => Ok(Json(AppResponse::new(
            new_player,
            Some("Playe created successfully!".to_string()),
            StatusCode::CREATED.as_u16() as i32,
            true,
        ))),
        Err(e) => Err(AppErrorType::from(e)),
    }
}

// #[post("/{player_id}")]
// async fn retrieve() {
    
// }