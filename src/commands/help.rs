use regex::Regex;
use rustbot::util::configuration::get_bot_prefix;
use rustbot::util::template::template_reader;
/// Bot help
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use tracing::{error, info};

///
/// Shows a generic help response
async fn show_generic_help(ctx: &Context, msg: &Message, bot_prefix: String) -> CommandResult {
    info!("Responding with generic help embed");
    // Show regular help
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("RustBot Help");
            e.description(format!("For more information on each command, you can use `{}help` command for a more specific description", bot_prefix));
            e.field(format!("{}run", bot_prefix), "Runs Rust code that you provide and responds to you with the output", false);
            e.field(format!("{}script", bot_prefix), "Runs rust script that you provide and responds to you with the output", false);
            e.field(format!("{}ping", bot_prefix), "Sanity check to test if the bot is active", false);
            e.field(format!("{}help", bot_prefix), "Shows this help page", false);

            e
        });
        m
    })
    .await;
    if let Err(why) = msg {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    info!("PING command from {}", msg.author.name);

    let bot_prefix = get_bot_prefix();
    let re = Regex::new(r"!help (.*)").unwrap();
    // input: "!help run"
    // matches whole string
    // capture group 1: run
    let captures = re.captures(msg.content.as_str());

    match captures {
        Some(cmd_capture) => {
            match cmd_capture.get(1).map(|m| String::from(m.as_str())) {
                Some(cmd) => {
                    // We have a command to show help for
                    let cmd_help = template_reader(format!("help_{}", cmd).as_str());
                    info!("Rendering help for {}", cmd);
                    match cmd_help {
                        Some(help_text) => {
                            // Render help text
                            let msg = msg
                                .channel_id
                                .send_message(&ctx.http, |m| {
                                    m.embed(|e| {
                                        e.title(format!("{}{} help", bot_prefix, cmd));
                                        e.description(help_text);

                                        e
                                    });
                                    m
                                })
                                .await;
                            if let Err(why) = msg {
                                println!("Error sending message: {:?}", why);
                            }
                        }
                        None => {
                            error!("Could not find information for command {}", cmd);
                            msg.reply(
                                &ctx.http,
                                "Sorry, I could not find help information for that command.",
                            )
                            .await?;
                        }
                    }
                }
                None => {
                    show_generic_help(ctx, msg, bot_prefix).await?;
                }
            }
        }
        None => {
            show_generic_help(ctx, msg, bot_prefix).await?;
        }
    }

    Ok(())
}
