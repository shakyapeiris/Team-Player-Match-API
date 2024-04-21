use log::{error, info};

use actix_web::{
    delete, get, post, put,
    web::{self, Json, Path},
    Responder,
};
use ulid::Ulid;

use crate::{
    models::{Player, Team, TeamBody},
    utils::{item_to_value, AppErrorType, AppResponse, AppState},
};

#[get("/")]
pub async fn list(
    app_state: web::Data<AppState>,
) -> Result<Json<AppResponse<Vec<Team>>>, AppErrorType> {
    let res = app_state.ddb.get_teams().await?;
    let mut res_vec = Vec::new();

    for i in res {
        res_vec.push(Team {
            PK: item_to_value("PK", &i).expect("Invalid key").unwrap(),
            SK: item_to_value("SK", &i).expect("Invalid key").unwrap(),
            name: item_to_value("name", &i).expect("Invalid key").unwrap(),
            code: item_to_value("code", &i).expect("Invalid key").unwrap(),
        })
    }

    Ok(Json(AppResponse::new(res_vec, None, 200, true)))
}

#[get("/{team_id}")]
pub async fn retrieve(
    app_state: web::Data<AppState>,
    team_id: Path<String>,
) -> Result<Json<AppResponse<Team>>, AppErrorType> {
    let res = app_state.ddb.get_team(&team_id).await?;
    if res.is_none() {
        return Err(AppErrorType::NotFoundError(
            "Couldn't find an item of give id".to_string(),
        ));
    }
    let res = res.unwrap();
    let team = Team {
        PK: item_to_value("PK", &res)
            .expect("Invalid key")
            .unwrap_or_default(),
        SK: item_to_value("SK", &res)
            .expect("Invalid key")
            .unwrap_or_default(),
        name: item_to_value("name", &res)
            .expect("Invalid key")
            .unwrap_or_default(),
        code: item_to_value("code", &res)
            .expect("Invalid key")
            .unwrap_or_default(),
    };
    Ok(Json(AppResponse::new(team, None, 200, true)))
}

#[post("/")]
pub async fn create(
    app_state: web::Data<AppState>,
    team: Json<TeamBody>,
) -> Result<Json<AppResponse<Team>>, AppErrorType> {
    let ulid = Ulid::new();
    let pk_and_sk =format!("T#{}", ulid.to_string());
    let new_team = Team {
        PK: pk_and_sk.to_string(),
        SK: pk_and_sk.to_string(),
        name: team.name.to_string(),
        code: team.code.to_string(),
    };
    let result = app_state.ddb.put_team(new_team.clone()).await;
    match result {
        Ok(_) => {
            let resp = AppResponse::new(
                new_team,
                Some("Team successfully created".to_string()),
                201,
                true,
            );
            info!("{:#?}", resp);
            Ok(Json(resp))
        }
        Err(e) => {
            error!("{:#?}", e.raw_response().unwrap().body());
            Err(AppErrorType::from(e))
        }
    }
}

#[put("/{team_id}")]
pub async fn update(team_id: Path<String>) -> impl Responder {
    format!("Update team {}", team_id)
}

#[delete("/{team_id}")]
pub async fn delete(team_id: Path<String>) -> impl Responder {
    format!("Delte team {}", team_id)
}

#[get("/{team_id}/players")]
pub async fn get_team_players(app_state: web::Data<AppState>,team_id: Path<String>) -> Result<Json<AppResponse<Vec<Player>>>, AppErrorType> {
    let result = app_state.ddb.get_team_player(format!("T#{}", team_id).as_str()).await;
    match result {
        Ok(team_players) => {
            let mut res_players = vec![];
            for i in team_players {
                res_players.push(Player {
                    PK: item_to_value("PK", &i).expect("Invalid Key").unwrap(),
                    SK: item_to_value("SK", &i).expect("Invalid Key").unwrap(),
                    fname: item_to_value("fname", &i).expect("Invalid Key").unwrap(),
                    lname: item_to_value("lname", &i).expect("Invalid Key").unwrap(),
                    photo: item_to_value("photo", &i).expect("Invalid Key").unwrap(),
                });
            }
            let resp = AppResponse::new(
                res_players,
                Some("Test message".to_string()),
                200,
                true,
            );

            Ok(Json(resp))
        },
        Err(e) => {
            error!("{:#?}", e);
            Err(AppErrorType::from(e))
        }
    }
}
