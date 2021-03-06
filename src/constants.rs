use std::collections::HashMap;

/// The ✅ emoji code in discord - used to indicate everything's peachy
pub const CHECK_MARK_EMOJI: char = '✅';

/// The  emoji code in discord - used to indicate something blew up
pub const CROSS_MARK_EMOJI: char = '❌';

/// The 🔨 emoji code in discord - used to indicate compilation started
pub const HAMMER_EMOJI: char = '🔨';

/// The ⏰ emoji in discord - used to indicate the command has timed out
pub const CLOCK_EMOJI: char = '⏰';

/// Environment variable for our bot token
pub const ENV_BOT_TOKEN: &str = "DISCORD_TOKEN";

/// Environment variable for max container runtime
pub const ENV_MAX_RUNTIME: &str = "MAX_CONTAINER_RUNTIME";
pub const DEFAULT_CONTAINER_RUNTIME: u64 = 5000;

/// Environment variable for bot prefix
pub const ENV_BOT_PREFIX: &str = "BOT_PREFIX";
pub const DEFAULT_PREFIX: &str = "!";

/// Base path to look for templates
pub const TEMPLATE_BASE_PATH: &str = "assets/templates";

// --------------------------- //
// CONTAINER RESOURCE SETTINGS //
// --------------------------- //
pub const ENV_CONTAINER_IMAGE: &str = "CONTAINER_IMAGE";
pub const DEFAULT_CONTAINER_IMAGE: &str = "ghcr.io/theconner/rustbot-runner:latest"; // Used only on release build
pub const DEFAULT_LOCAL_CONTAINER_IMAGE: &str = "rustbot-runner:latest";

pub const ENV_CONTAINER_CPU: &str = "CONTAINER_CPU";
pub const DEFAULT_CONTAINER_CPU: &str = "0.5"; // you get 1/2 of a cpu, i'm being generous

// TODO: add settings for CPU scheduler
// although, i'm unsure if podman supports userspace containers with different schedulers
pub const ENV_CONTAINER_MEMORY: &str = "CONTAINER_MEMORY";
pub const DEFAULT_CONTAINER_MEMORY: &str = "100m";

pub const ENV_CONTAINER_SWAP: &str = "CONTAINER_SWAP";
pub const DEFAULT_CONTAINER_SWAP: &str = "5m";

// Tells RustBot if it's running in a container
// this will influence flags it chooses for child containers
// available values: false,true
pub const ENV_IS_RUNNING_IN_CONTAINER: &str = "IS_RUNNING_IN_CONTAINER";
pub const DEFAULT_IS_RUNNING_IN_CONTAINER: bool = false;

// DEFAULT SETTINGS HASH MAP
lazy_static! {
    pub static ref DEFAULT_SETTINGS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(ENV_BOT_TOKEN, "NO_TOKEN_PROVIDED");
        m.insert(ENV_BOT_PREFIX, DEFAULT_PREFIX);
        m.insert(ENV_CONTAINER_IMAGE, DEFAULT_CONTAINER_IMAGE);
        m.insert(ENV_CONTAINER_CPU, DEFAULT_CONTAINER_CPU);
        m.insert(ENV_CONTAINER_MEMORY, DEFAULT_CONTAINER_MEMORY);
        m.insert(ENV_CONTAINER_SWAP, DEFAULT_CONTAINER_SWAP);
        m
    };
}
