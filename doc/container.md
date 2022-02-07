# RustBot Container

We ship two container images:
- `rustbot`: Responsible for hosting the bot itself
- `rustbot-runner`: Responsible for running code that is passed to it, invoked by `rustbot`

RustBot can be ran standalone as a user service, or it can be ran using the `rustbot` container image. Note that due to limitations of podman (which we use for rootless containers), we cannot impose resource restrictions to nested containers. In other words if a `rustbot-runner` is invoked by a `rustbot` container, we can't limit the amount of CPU and memory available to the runner container. If you run `rustbot` standalone, the CPU and memory limits will be applied.

Running the RustBot container is very simple:
```bash
podman run --security-opt label=disable \
 --user podman \
 --device /dev/fuse \
 -e DISCORD_TOKEN="YOUR_TOKEN_HERE" \
 ghcr.io/theconner/rustbot:latest
```

The above runs a rootless container that is capable of running nested rootless podman containers (ran whenever a `!run` command is received). For more information on nested containers using podman, see this excellent article on [how to use Podman inside of a container](https://www.redhat.com/sysadmin/podman-inside-container).

## Building the RustBot Container
Here is how to build the RustBot container. Note that the container only uses release builds.

1. Make release build of project `cargo build --release`
2. Strip debug symbols from release `strip -s target/release/*rustbot`
3. Generate RPM `cargo generate-rpm` 
4. Build container`podman build -f bot.Dockerfile -t rustbot:latest .`

Then, to run locally, substitute `ghcr.io/theconner/rustbot:latest` with `localhost/rustbot:latest`