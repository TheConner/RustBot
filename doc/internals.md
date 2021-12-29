## Internals

All code compilation and execution is done in a container, which runs in userspace via [Podman](https://podman.io/) instead of Docker. It's worth noting that containers are not shared, the container only lives for the lifespan of the `!run` command. 

When your `!run` command followed by rust code is received by the bot, the bot will extract your code from the message and then base64 encode it. We *could* be very pedantic about escaping your code when it gets transferred to the container to prevent breakouts; however, base64ing it does that for us for free. The encoded string is then ran by a [trampoline](assets/container/trampoline) in the container which handles decoding and running your program.

The output of this would be the output of the rust program, which is ran by the helpful [runner](https://docs.rs/crate/runner/latest) which is great for running standalone rust files without scaffolding out a full project.

To prevent DOS attacks on the platform, we have some policies to prevent code like this from eating up our resources:
```rs
use std::{thread, time};
fn main() {
    // do nothing for a minute
    thread::sleep(time::Duration::from_millis(60000));
    println!("Hello rustbot!");
}
```
Note that if you are using a podman-in-podman setup by running RustBot as a container, then the child containers spawned by RustBot cannot be limited 