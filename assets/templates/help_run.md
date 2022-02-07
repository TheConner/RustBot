`{{BOT_PREFIX}}run`: Runs some rust code. The code you give should be in a code block (see below) with `rs` identifying the language. For example to run a simple hello world program you would type

{{BOT_PREFIX}}run
\```rs
fn main() {
    println!("Hello shellbot!");
}
\```

If you want to pass arguments to the program, those can be included after the run command. For example:

{{BOT_PREFIX}}run hello world
\```rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
\```