use serenity::{
    model::prelude::*,
    http::Http,
    async_trait,
    client::{Context, EventHandler, Client},
    model::gateway::Ready,
    prelude::GatewayIntents,
};
use tokio::time::Duration; // For async sleep
use chrono::{Local, Timelike};
use std::fs;
use std::thread;

const PATH: &str = "/Users/jpcst/Documents/rust/discord-rust-bot/src/log.txt"; // Log contents to send as message
const TOKEN: &str = "x"; // Replace with your actual bot token
const USER_ID: u64 = 1; // Replace with your actual user ID

// Sleep time calculation
fn time_to_sleep(t: u64) -> u64 {
    match t % 15 {
        remainder => 15 - remainder,
    }
}

async fn sleep(t: u64) {
    let duration = Duration::from_secs(t * 60);
    thread::sleep(duration);
}

// Read the log txt file
fn read_log_file(dir: &str) -> String {
    fs::read_to_string(dir).expect("Error reading log file.")
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        // println!("{} is online", ready.user.name);
        // Set bot presence to online
        ctx.set_presence(Some(Activity::playing("with the Log Lady")), OnlineStatus::Online).await;
    }
}

// first message will be sent when the script runs
// then always repeat at the minutes 0-15-30-45-0 ...
#[tokio::main]
async fn main() {
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(TOKEN, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    let _ = tokio::spawn(async move {
        match client.start().await {
            Ok(_) => {}, // Successfully started the client
            Err(why) => println!("Client error: {:?}", why),
        }
    });

    loop {
        run_bot().await;
        // calculations for the waiting time
        let now = Local::now();
        let current_minute = now.minute().into();
        let t = time_to_sleep(current_minute);
        println!("Script will restart at minute {}\nPlease wait 00:{:02}", current_minute + t, t);
        sleep(t).await;
    }
}

async fn run_bot() {
    let http = Http::new(&TOKEN);
    let user = UserId(USER_ID);

    let log = read_log_file(PATH); // log file to send contents as message
    println!("\nCurrent log:\n{}\n", log);

    // send message to user
    match user.create_dm_channel(&http).await {
        Ok(channel) => {
            match channel.say(&http, log).await {
                Ok(_) => println!("Message sent successfully\n"),
                Err(why) => println!("Error sending message: {:?}", why),
            }
        },
        Err(why) => println!("Error creating DM channel: {:?}", why),
    }
}
