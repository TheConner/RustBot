use std::io;
use std::io::Error;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use regex::Regex;
use process_control::{ChildExt,Timeout,Output};
use crate::model::container::{RuntimeSettings};
use crate::util::configuration::{get_container_settings};

/// Extracts source code from a command message.
/// If no code is extractable, None is returned
/// Otherwise, you will get Some string
pub fn extract_code(text: &String) -> Option<String> {
    // <begin rant>
    // The regex I want to use is (?<=```)(rs|.*)((.|\n)*)(?=```)
    // but no lookahead or lookbehind support in rust's regex lib
    // which is TRASH (kidding)
    //
    // The regex I'm stuck with is (```)(rs|.*)((.|\n)*)(```) which
    // would require more massaging after processing.
    // it will have to do until the regex maintainers realize that
    // these "feature limitations" put rust's regex engine on par
    // with the same one used in the Safari web browser (also trash)
    // </end rant>
    let re = Regex::new(r"(```)(rs|.*)((.|\n)*)(```)").unwrap();
    let captures = re.captures(text);

    if captures.is_none() {
        // No matches found, return none
        return None;
    }
    // Have matches, return some
    let code_capture = captures.unwrap().get(3);
    return code_capture.map(|m| String::from(m.as_str()));
}

/// Builds a command to invoke our container with a command (cmd)
pub fn build_container_command(cmd: &str) -> String {
    let container_settings = get_container_settings();
    return format!("podman run {} rustbot:latest {}", 
    container_settings.generate_runtime_flags(), cmd);
}

/// Provides a uniform way of running a command with a timeout
pub async fn run_command_with_timeout(cmd: &str, timeout: u64) -> Result<Output, Error> {
    // Because std::command does not give me the ability to override / modify 
    // how arguments are escaped I have to do some stupid hack to make this 
    // work. For example, if I wanted to run
    // podman run rustbot:latest ls -al
    // this would be impossible if I did
    //
    //  std::process::Command::new("podman")
    //    .args(["run", "rustbot:latest", "ls -al"])
    //    .output()
    //    .expect("failed to invoke container");
    //
    // As the ls -al would be quoted, and the container would try to execute 
    // `ls -al` which would fail. The alternative is to seperate "ls", "-al" 
    // which would also fail as the container would run `ls` then `-al` 
    // ... what a stupid design
    // So instead of embracing the safety this API gives you, i'm just invoking
    // a shell with a payload I deem as safe
    let process = Command::new("sh")
        .args(["-c", cmd])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    
    let output = process
        .with_output_timeout(Duration::from_millis(timeout))
        .terminating()
        .wait()?
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::TimedOut, "Process timed out")
        })?;

    return Ok(output);
}