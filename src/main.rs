use serde::Deserialize;
use serenity::all::{CreateAttachment, CreateMessage, GatewayIntents};
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{
    CommandError, CommandResult, Configuration, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::{async_trait, Client};
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
struct Data {
    config: Config,
}
#[derive(Deserialize)]
struct Config {
    token: String,
    prefix: String,
}

struct Attachment {
    data: Vec<u8>,
    filename: String,
}

struct Resources {
    gimper: Attachment,
}

impl serenity::prelude::TypeMapKey for Resources {
    type Value = Resources;
}

fn read_config() -> Config {
    let config_file = "config.toml";

    let contents = fs::read_to_string(config_file)
        .expect(&format!("Config file: {} not found.", config_file));

    toml::from_str::<Data>(&contents).expect("").config
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f =
        File::open(&filename).expect(&format!("File: {} not found", filename));
    let metadata = fs::metadata(&filename)
        .expect(&format!("Unable to read file: {}", filename));
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect(&format!(
        "Unable to read file: {} buffer overflow",
        filename
    ));

    buffer
}

// registers global app resources
async fn create_resources(client: &mut Client) {
    let gimper = Attachment {
        data: get_file_as_byte_vec(&String::from("./img/gimper.jpg")),
        filename: String::from("gimper.jpg"),
    };
    let resources = Resources { gimper: gimper };

    let mut data = client.data.write().await;
    data.insert::<Resources>(resources);
}

#[group]
#[commands(ping)]
#[commands(gimper)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let config = read_config();
    let token = config.token;

    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix(config.prefix));

    let intents =
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // create app resources
    create_resources(&mut client).await;

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    match msg.reply(ctx, "Pong!").await {
        Ok(_) => Ok(()),
        Err(e) => Err(CommandError::from(e)),
    }
}

#[command]
async fn gimper(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let resources = data.get::<Resources>().unwrap();
    let gimper_attachment = CreateAttachment::bytes(
        resources.gimper.data.clone(),
        &resources.gimper.filename,
    );

    let builder = CreateMessage::new().add_file(gimper_attachment);
    match msg.channel_id.send_message(&ctx.http, builder).await {
        Ok(_) => Ok(()),
        Err(e) => Err(CommandError::from(e)),
    }
}