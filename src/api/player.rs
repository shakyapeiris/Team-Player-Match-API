use actix_web::{
    http::StatusCode,
    post,
    get,
    web::{self, Json, Path},
};
use ulid::Ulid;

use crate::{
    models::{Player, PlayerBody, Score},
    utils::{item_to_value, AppErrorType, AppResponse, AppState},
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

#[get("/{player_id}")]
async fn retrieve(player_id: Path<String>, web_data: web::Data<AppState>) -> Result<Json<AppResponse<Player>>, AppErrorType> {
    let player_res = web_data.ddb.get_player(format!("P#{}", player_id).as_str()).await;
    match player_res {
        Ok(player) => {
            let player = player.unwrap();
            let res = Player {
                PK: item_to_value("PK", &player)
                    .expect("Invalid key")
                    .unwrap_or_default(),
                SK: item_to_value("SK", &player)
                    .expect("Invalid key")
                    .unwrap_or_default(),
                fname: item_to_value("fname", &player)
                    .expect("Invalid key")
                    .unwrap_or_default(),
                lname: item_to_value("lname", &player)
                    .expect("Invalid key")
                    .unwrap_or_default(),
                photo: item_to_value("photo", &player)
                    .expect("Invalid key")
                    .unwrap_or_default(),
            };

            Ok(Json(AppResponse::new(res, Some("Successfull".to_string()), StatusCode::ACCEPTED.as_u16() as i32, true)))
        },
        Err(e) => Err(AppErrorType::from(e))
    }
}

#[get("/{player_id}/history")]
async fn retrieve_player_history(app_state: web::Data<AppState>, player_id: Path<String>) -> Result<Json<AppResponse<Vec<Score>>>, AppErrorType> {
    let result = app_state
        .ddb
        .get_player_history(format!("P#{}", player_id))
        .await?;
    let mut res_vec = Vec::new();

    for item in result {
        res_vec.push(Score {
            PK: item_to_value("PK", &item).expect("Invalid Key").unwrap(),
            SK: item_to_value("SK", &item)
                .expect("Invalid Key")
                .unwrap(),
            team: item_to_value("team", &item)
                .expect("Invalid Key")
                .unwrap(),
            player: item_to_value("player", &item)
                .expect("Invalid Key")
                .unwrap(),
        })
    }

    Ok(Json(AppResponse::new(res_vec, None, 200, true)))
}