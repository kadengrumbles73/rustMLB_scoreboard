use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MlbSchedule{
    #[serde(rename = "totalGames")]
    pub total_games: i32,
    #[serde(rename = "totalGamesInProgress")]
    pub total_games_in_progress: i32,
    pub dates: Vec<ScheduleDate>, // gets the games from selected date
}

#[derive(Deserialize, Debug)]
pub struct ScheduleDate{
    pub date: String, // (2026-05-20)
    pub games: Vec<Game>, // list of games from game struct
}

// sees if game live, final or upcoming
#[derive(Deserialize, Debug)]
pub struct Game{
    #[serde(rename = "gamePk")] 
    pub game_id: i32,
    pub status: GameStatus,  // point to struct
    pub teams:  Teams,   // points to teams stru
}

#[derive(Deserialize, Debug)]
pub struct GameStatus{
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

#[derive(Deserialize, Debug)]
pub struct Teams{
    // rust will map the left most one first so away first
    pub away: TeamDetail, //gets info from other struct
    pub home: TeamDetail,
}

#[derive(Deserialize, Debug)]
pub struct TeamDetail{
    pub score: Option<i32>,
    pub team: TeamInfo,
    //pub wins: i32, // wont work they have a sub struct i will work this update later
    //pub losses: i32,
}

#[derive(Deserialize, Debug)]
pub struct TeamInfo {
    pub name: String,
}