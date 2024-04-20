use std::collections::HashMap;

use aws_sdk_dynamodb::{
    error::SdkError,
    operation::{put_item::PutItemError, query::QueryError, scan::ScanError},
    types::AttributeValue,
    Client,
};
use log::{error, info};

use crate::models::{Game, Player, Score, Team};
pub struct DDB {
    client: Client,
    table_name: String,
}

impl DDB {
    pub async fn new(table_name: &str) -> DDB {
        let config = aws_config::load_from_env().await;
        // let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config)
        //     .endpoint_url("http://localhost:8000")
        //     .build();
        // let client = Client::from_conf(dynamodb_local_config);
        let client = Client::new(&config);

        DDB {
            client,
            table_name: table_name.to_string(),
        }
    }

    pub async fn get_teams(
        &self,
    ) -> Result<Vec<HashMap<std::string::String, AttributeValue>>, SdkError<ScanError>> {
        let result = self
            .client
            .scan()
            .table_name(self.table_name.to_string())
            .send()
            .await;
        match result {
            Ok(output) => Ok(output.items.unwrap()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_team(
        &self,
        id: &str,
    ) -> Result<Option<HashMap<String, AttributeValue>>, SdkError<QueryError>> {
        let result = self
            .client
            .query()
            .table_name(self.table_name.to_string())
            .key_condition_expression("PK = :teamid")
            .expression_attribute_values(
                ":teamid",
                AttributeValue::S(format!("team#{}", id.to_string())),
            )
            .send()
            .await;

        match result {
            Ok(item) => Ok(match item.items.unwrap().first() {
                Some(i) => Some(i.clone()),
                None => None,
            }),
            Err(e) => {
                println!("{:#?}", e);
                Err(e)
            }
        }
    }

    pub async fn put_team(&self, team: Team) -> Result<(), SdkError<PutItemError>> {
        info!("Creating team...");
        self.client
            .put_item()
            .table_name(self.table_name.to_string())
            .item("PK", AttributeValue::S(team.PK.to_string()))
            .item("SK", AttributeValue::S(team.SK.to_string()))
            .item("name", AttributeValue::S(team.name))
            .item("code", AttributeValue::S(team.code))
            .send()
            .await?;
        Ok(())
    }

    pub async fn put_player(&self, player: Player) -> Result<(), SdkError<PutItemError>> {
        self.client
            .put_item()
            .table_name(self.table_name.to_string())
            .item("PK", AttributeValue::S(player.PK))
            .item("SK", AttributeValue::S(player.SK))
            .item("fname", AttributeValue::S(player.fname))
            .item("lname", AttributeValue::S(player.lname))
            .item("photo", AttributeValue::S(player.photo))
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_team_player(
        &self,
        team_id: &str,
    ) -> Result<Vec<HashMap<std::string::String, AttributeValue>>, SdkError<QueryError>> {
        let result = self
            .client
            .query()
            .table_name(self.table_name.to_string())
            .key_condition_expression("PK = :team_id and begins_with(SK, :player_id)")
            .expression_attribute_values(":team_id", AttributeValue::S(team_id.to_string()))
            .expression_attribute_values(":player_id", AttributeValue::S("P".to_string()))
            .send()
            .await?;

        Ok(result.items.unwrap())
    }

    pub async fn put_match(&self, game: Game) -> Result<(), SdkError<PutItemError>> {
        self.client
            .put_item()
            .table_name(self.table_name.to_string())
            .item("PK", AttributeValue::S(game.PK))
            .item("SK", AttributeValue::S(game.SK))
            .item("title", AttributeValue::S(game.title))
            .item("teams", AttributeValue::Ss(game.teams))
            .item("date", AttributeValue::S(game.date))
            .item("time", AttributeValue::S(game.time))
            .item("venue", AttributeValue::S(game.venue))
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_match(
        &self,
        game_id: String,
    ) -> Result<HashMap<String, AttributeValue>, SdkError<QueryError>> {
        let result = self
            .client
            .query()
            .table_name(self.table_name.to_string())
            .key_condition_expression("PK = :game_id and SK=:game_id")
            .expression_attribute_values(":game_id", AttributeValue::S(game_id))
            .send()
            .await;

        match result {
            Ok(result) => Ok(result.items.unwrap().first().unwrap().to_owned()),
            Err(e) => {
                error!("{:#?}", e);
                Err(e)
            }
        }
    }

    pub async fn put_score(&self, score_body: Score) -> Result<(), SdkError<PutItemError>> {
        let result = self
            .client
            .put_item()
            .table_name(self.table_name.to_string())
            .item("PK", AttributeValue::S(score_body.PK))
            .item("SK", AttributeValue::S(score_body.SK))
            .item("player", AttributeValue::S(score_body.player))
            .item("team", AttributeValue::S(score_body.team))
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}
