// RustBot main entry point
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::{macros::group, StandardFramework},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use std::{collections::HashSet, process, sync::Arc};
use tracing::{error, info, instrument};

use rustbot::constants::ENV_BOT_TOKEN;
use rustbot::util::command::pull_latest_container_image;
use rustbot::util::configuration::*;

mod commands;
use commands::help::*;
use commands::ping::*;
use commands::run::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(ping, run, help)]
struct General;

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("RustBot: starting up");

    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().ok();

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to `debug`.
    //tracing_subscriber::fmt::init();

    let token = get_bot_token();

    if token.is_empty() {
        error!(
            "ERROR: no token provided! Please set the {} environment variable",
            ENV_BOT_TOKEN
        );
        process::exit(0x0100);
    }

    // Pull the container image in advance, fail if it fails
    if !is_debug() {
        let container_pull_result = pull_latest_container_image().await;
        match container_pull_result {
            Ok(_res) => info!("Container pull OK"),
            Err(why) => {
                error!("Could not pull container image, got error code {}", why);
                process::exit(0x0100);
            }
        };
    }

    let http = Http::new(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(get_bot_prefix().as_str())
            .owners(owners))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES 
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    {
        info!("Setting up shard manager");
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
