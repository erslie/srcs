

use std::fs;
use std::{collections::HashSet, fs::File, io::BufReader, usize};
use serenity::model::gateway::Ready;
use serenity::{async_trait, framework, http};
use serenity::framework::standard::{
    macros::{group, help},
};
use serenity::model::{channel::Message, id::UserId};
use serenity::prelude::{Client, Context, EventHandler};

use serde::{Deserialize, Serialize};
use serde_json::Result;
use serenity::all::GatewayIntents;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
    
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!yaju" {
            msg.channel_id.say(&ctx.http, format!("hello, {}! ", msg.author)).await.expect("error");
        }
    }
}


#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

fn get_token(file_name: &str) -> Result<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let t: Token = serde_json::from_reader(reader).unwrap();
    Ok(t.token)
}

#[tokio::main]
async fn main() {
    let token = get_token("config.json").expect("err");
    let intents = GatewayIntents::all();

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let body = get_reqwest();
    println!("body is \n{}", body.await.to_string());


    if let Err(why) = client.start().await {
        println!("Client error : {:?}", why);
    }

}

const SDVXIN: &str = "https:/sdvx.in/sort/sort_18.htm";
async fn get_reqwest() -> String {
    let body = reqwest::get(SDVXIN).await.unwrap().text().await.unwrap();
    body
}

//atode...
// async fn scraping(target: &str) -> String {
//     let document = scraper::Html::parse_document(target);

//     let selector = scraper::Selector::parse("")
// }