use rustbot::constants::{CHECK_MARK_EMOJI, CLOCK_EMOJI, CROSS_MARK_EMOJI, HAMMER_EMOJI};
use rustbot::util::command::{build_container_command, extract_code, run_command_with_timeout};
use rustbot::util::configuration::get_container_runtime;
use rustbot::util::template::template_reader;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use std::io::ErrorKind::TimedOut;
use tracing::{debug, info};

/// Given some stdout or stderr data, format it so that it can be rendered by discord
fn response_formatter(response: String) -> String {
    debug!("Format response \"{}\"", response);
    if response.len() < 1990 {
        // Response falls within size constraints
        return format!("```\n{}\n```", response);
    } else {
        // we trim to 1981 chars because [TRIMMED] is 9 chars
        let short_repsonse = &response[0..1981]; // TODO: maybe do this in place with a mutable string
        return format!("```{}[TRIMMED]```", short_repsonse);
    }
}

#[command]
pub async fn run(ctx: &Context, msg: &Message) -> CommandResult {
    let code = extract_code(&msg.content);
    let code_author = &msg.author.name;
    info!("Running message from {}", msg.author.name);
    // TODO: tidy this up
    match code {
        Some(c) => {
            msg.react(ctx, HAMMER_EMOJI).await?;

            // With the code matched, we have to b64 encode it to be sent to the container
            // the `payload` will then be encoded and decoded inside the container in a similar fashion to the original ShellBot
            let encoded_program = base64::encode(c.code.unwrap_or_else(|| String::from("")));
            let encoded_args = base64::encode(c.args.unwrap_or_else(|| String::from("")));
            let payload = build_container_command(
                format!("trampoline {} {}", encoded_program, encoded_args).as_str(),
            );

            debug!("Trampoline Payload \"{}\"", payload);

            let cmd_result =
                run_command_with_timeout(payload.as_str(), get_container_runtime()).await;

            match cmd_result {
                Ok(output) => {
                    let mut stdout = String::new();
                    let mut stderr = String::new();

                    if !output.stdout.is_empty() {
                        stdout = match String::from_utf8(output.stdout) {
                            Ok(v) => v,
                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        };

                        debug!("Got stdout\n\"{}\"", stdout);
                    } else {
                        debug!("No stdout");
                    }

                    if !output.stderr.is_empty() {
                        stderr = match String::from_utf8(output.stderr) {
                            Ok(v) => v,
                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        };
                        debug!("Got stderr \"{}\"", stderr);
                    } else {
                        debug!("No stderr");
                    }

                    // Check to see if the response was nothing
                    if !stdout.is_empty() && stderr.is_empty() {
                        debug!("Response: has stdout, no stderr");
                        msg.react(&ctx, CHECK_MARK_EMOJI).await?;
                        msg.reply(&ctx, response_formatter(stdout)).await?;
                    } else if stdout.is_empty() && !stderr.is_empty() {
                        debug!("Response: no stdout, has stderr");
                        // Had stderr, no stdout
                        msg.react(&ctx, CROSS_MARK_EMOJI).await?;
                        msg.reply(&ctx, response_formatter(stderr)).await?;
                    } else {
                        debug!("Response: no stdout, no stderr");
                        msg.react(&ctx, CHECK_MARK_EMOJI).await?;
                        msg.reply(
                            &ctx,
                            template_reader("run_no_output")
                                .expect("Could not read template run_no_output"),
                        )
                        .await?;
                    }
                }
                Err(error) => {
                    // TODO: find out ways this can blow up
                    info!("TIMEOUT on {}'s code", code_author);
                    match error.kind() {
                        TimedOut => {
                            // Took too long to run, complain to user
                            let response = template_reader("run_error_too_long")
                                .expect("Could not read template run_error_too_long");
                            msg.react(&ctx, CROSS_MARK_EMOJI).await?;
                            msg.react(&ctx, CLOCK_EMOJI).await?;
                            msg.reply(&ctx, response).await?;
                        }
                        _ => {
                            msg.react(ctx, CROSS_MARK_EMOJI).await?;
                            info!("Handled error {}", error)
                        }
                    }
                }
            }
        }
        None => {
            info!("NO CODE MATCH on {}'s message", code_author);
            // No code matched
            // show the help text
            let response =
                template_reader("help_running").expect("Could not read template help_running");
            msg.react(ctx, CROSS_MARK_EMOJI).await?;
            msg.reply(ctx, response).await?;
        }
    };

    Ok(())
}
