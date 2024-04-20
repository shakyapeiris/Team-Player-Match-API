use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerBody {
    pub team: String,
    pub fname: String,
    pub lname: String,
    pub photo: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub PK: String,
    pub SK: String,
    pub fname: String,
    pub lname: String,
    pub photo: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TeamBody {
    pub name: String,
    pub code: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Team {
    pub PK: String,
    pub SK: String,
    pub name: String,
    pub code: String
}



#[derive(Serialize, Deserialize)]
pub struct GameBody {
    pub title: String,
    pub teams: Vec<String>,
    pub venue: String,
    pub date: String,
    pub time: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub PK: String,
    pub SK: String,
    pub title: String,
    pub teams: Vec<String>,
    pub venue: String,
    pub date: String,
    pub time: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Score {
    pub PK: String, // game
    pub SK: String, // time, GSI1SK
    pub player: String, // GSI1PK
    pub team: String
}

// #[derive(Serialize, Deserialize, Clone)]