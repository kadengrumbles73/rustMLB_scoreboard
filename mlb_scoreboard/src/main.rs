mod mlb_api;
use serde_json;

fn main() {
    let url = "https://statsapi.mlb.com/api/v1/schedule?sportId=1";

    match reqwest::blocking::get(url){
        Ok(response) => {
            match response.text(){
                Ok(raw_json) =>  {
                    println!("CONNECTION SUCCESS");
                    // gets date from the struct 
                    let schedule_data: mlb_api::MlbSchedule = serde_json::from_str(&raw_json).unwrap();
                for date in schedule_data.dates{
                    // prints live date
                    println!("Date: {}", date.date);
                    // WILL GET ALL GAMES FROM TODAY
                    println!("Total Games Today: {}",  schedule_data.total_games);
                    // put games into categories
                    let mut live_count = 0;
                    let mut final_count = 0;
                    let mut upcoming_count = 0;

                    for game in date.games{
                        let home_team = &game.teams.home.team.name;
                        let away_team = &game.teams.away.team.name;
                        // get status from status struct
                        let status = &game.status.status_code;
                        
                        let status_text = format_status_text( // call status function wa
                                status, 
                                &mut live_count, 
                                &mut final_count, 
                                &mut upcoming_count
                            );

                        let home_score = &game.teams.home.score.unwrap_or(0);
                        let away_score = &game.teams.away.score.unwrap_or(0); // if score is null put 0
                        //print the scoreboard
                        println!("      [{}], {} ({}) vs. ({}) {}", status_text, away_team, away_score, home_score, home_team);

                    }
                    println!("Live Games:      {}", live_count);
                    println!("Finished Games:  {}", final_count);
                    println!("Upcoming Games:  {}", upcoming_count);
                }
                }
                Err(e) => {
                    eprintln!("Failed to read text: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

fn format_status_text(status: &str, live_count: &mut i32, final_count: &mut i32, upcoming_count: &mut i32) -> &'static str {
// to turn code into readable text
    if status == "I" {
        *live_count += 1;   //add game to live count
        "LIVE"
        }
        else if status == "F"{
            *final_count += 1;  // add game to final count
            "FINAL"
        }
        else if status == "P"{
            *upcoming_count += 1;  // add gam to upcoming count
            "WARMUP"
        }
        else{
            *upcoming_count += 1;  // add gam to upcoming count
            "SCHEDULED"
        }
}