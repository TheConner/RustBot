/// 
/// The fun part of this project, the command runner
/// 
use std::io::ErrorKind::{TimedOut};
use serenity::framework::standard::macros::{command};
use serenity::model::prelude::{Message};
use serenity::prelude::{Context};
use serenity::framework::standard::{CommandResult};
use rustbot::util::configuration::{get_container_runtime};
use rustbot::util::command::{run_command_with_timeout, extract_code, build_container_command};
use rustbot::util::template::{template_reader};
use rustbot::constants::{CHECK_MARK_EMOJI, CROSS_MARK_EMOJI, HAMMER_EMOJI, CLOCK_EMOJI};

/// Given some stdout or stderr data, format it so that it can be rendered by discord
fn response_formatter(response: String) -> String {
    if response.len() < 1990 {
        // Response falls within size constraints 
        return format!("```{}```", response);
    } else {
        // we trim to 1981 chars because [TRIMMED] is 9 chars
        let short_repsonse = &response[0..1981];  // TODO: maybe do this in place with a mutable string
        return format!("```{}[TRIMMED]```", short_repsonse);
    }
}

#[command]
pub async fn run(ctx: &Context, msg: &Message) -> CommandResult {
    let code = extract_code(&msg.content);

    // TODO: tidy this up
    match code {
        Some(c) => {
            msg.react(ctx, HAMMER_EMOJI).await?;

            // With the code matched, we have to b64 encode it to be sent to the container
            // the `payload` will then be encoded and decoded inside the container in a similar fashion to the original ShellBot
            let encoded = base64::encode(c);
            let payload = build_container_command(format!("trampoline {}", encoded).as_str());

            let cmd_result = run_command_with_timeout(payload.as_str(), get_container_runtime()).await;

            match cmd_result {
                Ok(output) => {
                    let mut stdout = String::new();
                    let mut stderr = String::new();
        
                    if output.stdout.len() > 0 {
                        stdout = match String::from_utf8(output.stdout) {
                            Ok(v) => v,
                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        };
                    }
        
                    if output.stderr.len() > 0 {
                        stderr = match String::from_utf8(output.stderr) {
                            Ok(v) => v,
                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        };
                    }
        
                    // Check to see if the response was nothing
                    if stdout.len() == 0 && stderr.len() == 0 {
                        // No stdout or stderr
                        msg.react(&ctx, CHECK_MARK_EMOJI).await?;
                        msg.reply(&ctx, template_reader("run_no_output").expect("Could not read template run_no_output")).await?;
                    } else if stdout.len() == 0 && stderr.len() > 0 {
                        // Had stderr, no stdout
                        msg.react(&ctx, CROSS_MARK_EMOJI).await?;
                        msg.reply(&ctx, response_formatter(stderr)).await?;
                    } else {
                        msg.react(&ctx, CHECK_MARK_EMOJI).await?;
                        msg.reply(&ctx, response_formatter(stdout)).await?;
                    }
                },
                Err(error) => { // TODO: find out ways this can blow up
                    match error.kind() {
                        TimedOut => {
                            // Took too long to run, complain to user
                            let response = template_reader("run_error_too_long").expect("Could not read template run_error_too_long");
                            msg.react(&ctx, CROSS_MARK_EMOJI).await?;
                            msg.react(&ctx, CLOCK_EMOJI).await?;
                            msg.reply(&ctx, response).await?;

                        },
                        _ => {
                            msg.react(ctx, CROSS_MARK_EMOJI).await?;
                            println!("Handled error {}", error)
                        }
                    }
                }
            }
        },
        None => {
            // No code matched
            // show the help text
            let response = template_reader("help_running").expect("Could not read template help_running");
            msg.react(ctx, CROSS_MARK_EMOJI).await?;
            msg.reply(ctx, response).await?;
        },
    };

    Ok(())
}