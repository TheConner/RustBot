#
# RustBot Docker Image
#
FROM rust:alpine

# Need dev utils for alpine
RUN apk add --no-cache musl-dev

# Need the rust runner utility
RUN cargo install runner

# Set up our user with restricted bash (rbash)
RUN adduser --disabled-password \
    --shell /bin/sh \
    --home /home/rustbot/ rustbot
USER rustbot
WORKDIR /home/rustbot
COPY assets/container/trampoline /bin/trampoline