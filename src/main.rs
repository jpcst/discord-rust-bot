use serenity::{model::prelude::*, http::Http};
use tokio;
use chrono::{Local, Timelike};
use std::{thread, fs, time::Duration};

// log file path with contents to send as message
const PATH: &str = "/Users/jpcst/Documents/rust/discord-rust-bot/src/log.txt";

// sleep X minutes for multiples of 15 minutes (15,30,45)
fn time_to_sleep(t: u64) -> u64 {
    match t % 15 {
        // 0 | 15 | 30 | 45 => 0,
        remainder => 15 - remainder,
    }
}

// sleep function for above calculation
fn sleep(t: u64) {
    let duration = Duration::from_secs(t * 60);
    thread::sleep(duration);
}

// read the log txt file
fn read_log_file(dir: &'static str) -> String {
    fs::read_to_string(dir).expect("Error reading log file.")
}

async fn run_bot() {
    let token: &str = ""; // discord bot API token
    let http = Http::new(&token);
    let user_id: u64 = ; // user ID to send DM
    let user = UserId(user_id);

    let log = read_log_file(PATH); // log file to send contents as message
    println!("\nCurrent log:\n{}\n", log);

    // send message to user
    match user.create_dm_channel(&http).await {
        Ok(channel) => {
            match channel.say(&http, log).await {
                Ok(_) => println!("Message sent successfully!\n"),
                Err(why) => println!("Error sending message: {:?}", why),
            }
        },
        Err(why) => println!("Error creating DM channel: {:?}", why),
    }
}

// first message will be sent when the script runs
// then always repeat at the minutes 0-15-30-45-0 ...
#[tokio::main]
async fn main() {
    loop {
        run_bot().await;
        // calculations for the waiting time
        let now = Local::now();
        let current_minute = now.minute().into();
        let t = time_to_sleep(current_minute);
        println!("Script will restart at minute {}\nPlease wait 00:{:02}", current_minute + t, t);
        sleep(t);
    }
}