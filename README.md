# RustBot
![CheckAndLint](https://github.com/TheConner/RustBot/actions/workflows/check.yaml/badge.svg)&nbsp;&nbsp;&nbsp;![ReleaseBuild](https://github.com/TheConner/RustBot/actions/workflows/release.yaml/badge.svg)

<p align="center">
  <img src="./assets/demo/rustbot_basic.png">
</p>

*Bot is still under development and not ready for production use*

RustBot is a discord bot that executes whatever rust code you throw at it. In a nutshell, it is remote code execution as a service 😛. Some practical applications of a bot that executes whatever code you throw at it is for Code Golf on discord servers (this bot only does rust), or for educational purposes where you could show code examples in a conversation. 

For more information on how RustBot works, see our [internals](doc/internals.md) page.

Future work for this bot includes:
- **Multitenancy/Admin commands**: Allow admins to configure various settings, this would involve some form of database integration and adding additional contexts to bot messages.



## Configuration
Want to run your own RustBot? Great! I only have instructions to get you started developing locally on your own machine. In the future I will provide instructions for server deployments.

1. You need [Podman](https://podman.io/) installed.
2. Clone this repo, and add the `.env` file with your token. See [our instructions](doc/discord.md) on how to make a token and add a bot to your server for local development
    ```
    DISCORD_TOKEN="YOUR_TOKEN_HERE"
    ```
3. Build the container by running `podman build -f Dockerfile_runner -t rustbot-runner:latest .`
4. OPTIONAL: if you are working on the container for rustbot itself, see the section on how to [build the container](doc/container.md)
5. Build and run this project with `cargo run`

### Environment Variables
These can either by specified by a `.env` file, or by exposing them the regular way.
| Name | Description | Required? | Default Value |
|------|-------------|-----------|---------------|
| `DISCORD_TOKEN` | Discord bot token | Required | |
| `BOT_PREFIX` | Prefix to use for commands | Optional | `!` |
| `MAX_CONTAINER_RUNTIME` | Max amount of milliseconds before the container is killed | Optional | 5000 | 
| `CONTAINER_CPU` | Max amount of CPU to delegate to the container | Optional | `0.5` | 
| `CONTAINER_MEMORY` | Max amount of memory available to the container | Optional | `100m` |
| `CONTAINER_SWAP` | Max amount of swap available to the container | Optional | `5m` |
| `CONTAINER_IMAGE` | Container image to use | Optional | Uses a local `rustbot-runner:latest` for dev builds, for release it uses the [ghcr.io container image](ghcr.io/theconner/rustbot-runner:latest) |
| `IS_RUNNING_IN_CONTAINER` | Tells RustBot if it's running as a container | Optional | False by default, True by default for our [container builds](Dockerfile_bot) |

## Bot Commands
More commands should be coming soon, here is what we support at the moment:

- `!run`: runs arbitrary code. The command expects there to be a code block. 

    For example:
    > !run
    > 
    > \```rs
    > 
    > fn main() { println!("Hello RustBot"); }
    >
    > \```
    
    Or, without the language identifier:
    > !run
    > 
    > \```
    >
    > fn main() { println!("Hello RustBot"); }
    >
    > \```

    The bot will react with `🔨` to indicate your code is building / being executed, a `✅` to indicate the run is successful, and `❌` to indicate something went wrong. A reply will be posted by the bot with the standard output of your code. For the above example the response would be

    > ```
    > Hello RustBot
    > ```

    For edge cases such as if a response is too long, the response will be truncated to fit Discord's max message length. 

- `!ping`: Checks if the bot is working. The bot will react to your message and respond with PONG.



## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
