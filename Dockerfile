#
# RustBot Docker Image
#
FROM rust:alpine

# Need the rust runner utility
RUN cargo install runner

# Set up our user with restricted bash (rbash)
RUN adduser --shell /bin/sh --home /home/rustbot/ rustbot
USER rustbot
WORKDIR /home/rustbot
COPY assets/container/trampoline /bin/trampoline