mod api;
mod models;
mod utils;

use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use utils::{ddb::DDB, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let ddb = DDB::new("team-players").await;
    let app_state = web::Data::new(AppState { ddb });

    HttpServer::new(move || {
        println!("App started on PORT: 8030");
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(app_state.clone())
            .service(
                web::scope("/teams")
                    .service(api::team::list)
                    .service(api::team::create)
                    .service(api::team::update)
                    .service(api::team::delete)
                    .service(api::team::retrieve)
                    .service(api::team::get_team_players),
            )
            .service(web::scope("/players").service(api::player::create))
            .service(
                web::scope("/matches")
                    .service(api::game::create)
                    .service(api::game::retrieve)
                    .service(api::game::post_team_score),
            )
    })
    .workers(1)
    .bind(("127.0.0.1", 8030))?
    .run()
    .await
}
