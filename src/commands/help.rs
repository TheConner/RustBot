/// Bot help
use serenity::framework::standard::macros::{command};
use serenity::model::prelude::{Message};
use serenity::prelude::{Context};
use serenity::framework::standard::{CommandResult};
use rustbot::util::template::{template_reader};
use rustbot::util::configuration::{get_bot_prefix};
use regex::Regex;

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let bot_prefix = get_bot_prefix();
    
    let re = Regex::new(r"!help (.*)").unwrap();
    // input: "!help run"
    // matches whole string
    // capture group 1: run
    let captures = re.captures(msg.content.as_str());

    let cmd_capture = captures
        .unwrap()
        .get(1)
        .map(|m| String::from(m.as_str()));

    match cmd_capture {
        Some(cmd) => {
            // We have a command to show help for
            let cmd_help = template_reader(format!("help_{}", cmd).as_str());

            match cmd_help {
                Some(help_text) => {
                    // Render help text
                    let msg = msg.channel_id.send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title(format!("{}{} help", bot_prefix, cmd));
                            e.description(format!("{}", help_text));
                
                            e
                        });
                        m
                    })
                    .await;
                    if let Err(why) = msg {
                        println!("Error sending message: {:?}", why);
                    }
                },
                None => {
                    msg.reply(&ctx.http, "Sorry, I could not find help information for that command.").await?;
                }
            }
        },
        None => {
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
                println!("Error sending message: {:?}", why);
            }
        }
    }

    
    Ok(())
}