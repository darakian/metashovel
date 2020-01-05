use regex::Regex;
use std::{thread, time};

fn main() {
    let consoles = vec![
        "xbox",
        "xbox360",
        "ds",
        "gba",
        "psp",
        "ps",
        "ps2",
        "ps3",
        "n64",
        "gamecube",
        "wii",
        "wii-u",
        "dreamcast",
    ];
    for c in consoles{
        let score = average_console_score(c);
        println!("Console: {:?}, Average Score: {}", c, score);
    }
}


fn average_console_score(choice: &str) -> f32{
    let backoff = time::Duration::from_millis(3600);
    let mut count = 0;
    let mut running_score = 0;
    let score_match = Regex::new(r">..").unwrap();

    for i in 0..30{
        thread::sleep(backoff);
        let url = format!("https://www.metacritic.com/browse/games/release-date/available/{}/date?page={}", choice, i);
        if i == 0{
            println!("{:?}", url);
        }
        let mut response = get_non_error_response(url.as_str());
        while response.status().as_u16() != 200{
            println!("Status {:?}", response.status().as_u16());
            if response.status().as_u16() == 404{
                continue
            }
            thread::sleep(backoff);
            response = get_non_error_response(url.as_str())
        }

        if response.status().is_server_error(){
            continue
        }
        if response.status().is_success(){
            let body = response.text().unwrap();
            for token in body.lines(){
                if token.contains("metascore_w"){
                    let cap = score_match.captures(token).unwrap();
                    let candidate_score = cap.get(0).unwrap().as_str().replace('>',"");
                    if candidate_score == "tb"{
                        continue;
                    }
                    let numeric_score: u32 = candidate_score.parse().unwrap_or(0);
                    running_score+=numeric_score;
                    count+=1;
                }
            }
        }
    }
    if count == 0{
        count+=1
    }
    (running_score/count) as f32
}

fn get_non_error_response(url: &str) -> reqwest::blocking::Response{
    let mut response = reqwest::blocking::get(url);
    while response.is_err(){
        response = reqwest::blocking::get(url);
    }
    response.unwrap()
}
