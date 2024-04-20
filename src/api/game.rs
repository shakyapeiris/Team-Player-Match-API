use crate::{
    models::{Game, GameBody, Score},
    utils::{item_to_value, item_to_vector, AppErrorType, AppResponse, AppState},
};
use actix_web::{
    body::{self, None}, delete, get,
    http::StatusCode,
    post, put,
    web::{self, Data, Json},
};
use ulid::Ulid;

#[get("/{game_id}")]
pub async fn retrieve(
    web_data: Data<AppState>,
    game_id: web::Path<String>,
) -> Result<Json<AppResponse<Game>>, AppErrorType> {
    let result = web_data.ddb.get_match(format!("G#{}", game_id).to_string()).await?;
    let res_data = Game {
        PK: item_to_value("PK", &result).expect("Invalid key").unwrap(),
        SK: item_to_value("SK", &result).expect("Invalid key").unwrap(),
        teams: item_to_vector("teams", &result)
            .expect("Invalid key")
            .unwrap(),
        venue: item_to_value("venue", &result)
            .expect("Invalid key")
            .unwrap(),
        title: item_to_value("title", &result)
            .expect("Invalid key")
            .unwrap(),
        date: item_to_value("date", &result)
            .expect("Invalid key")
            .unwrap(),
        time: item_to_value("time", &result)
            .expect("Invalid key")
            .unwrap(),
    };

    Ok(Json(AppResponse::new(
        res_data,
        Some("Team data".to_string()),
        StatusCode::ACCEPTED.as_u16() as i32,
        true,
    )))
}

pub async fn list() {

}

#[post("/")]
pub async fn create(
    web_data: web::Data<AppState>,
    body: Json<GameBody>,
) -> Result<Json<AppResponse<Game>>, AppErrorType> {
    let ulid = Ulid::new();
    let pk_and_sk = format!("G#{}", ulid.to_string());
    let game = Game {
        PK: pk_and_sk.clone(),
        SK: pk_and_sk.clone(),
        title: body.title.clone(),
        teams: body.teams.clone(),
        venue: body.venue.clone(),
        date: body.date.clone(),
        time: body.time.clone(),
    };

    web_data.ddb.put_match(game.clone()).await?;
    // web_data.ddb.get_match(pk_and_sk.to_string()).await;
    Ok(Json(AppResponse::new(
        game,
        Some("Match added successfully!".to_string()),
        StatusCode::CREATED.as_u16() as i32,
        true,
    )))
}

#[post("/score")]
pub async fn post_team_score(app_data: web::Data<AppState>, score_data: Json<Score>) -> Result<Json<AppResponse<Score>>, AppErrorType> {
    let conn = app_data.ddb.put_score(score_data.clone()).await;
    match conn {
        Ok(()) => {
            Ok(Json(AppResponse::new(
                Score {
                    PK: score_data.PK.clone(),
                    SK: score_data.SK.clone(),
                    player: score_data.player.clone(),
                    team: score_data.team.clone()
                },
                Some("Score added successfully!".to_string()),
                StatusCode::CREATED.as_u16() as i32,
                true,
            )))
        },
        Err(e) => {
            Err(AppErrorType::from(e))
        }
    }
    
}

pub async fn update() {}

pub async fn delete() {}
