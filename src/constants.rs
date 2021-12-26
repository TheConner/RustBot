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

// --------------------------- //
// CONTAINER RESOURCE SETTINGS //
// --------------------------- //
pub const ENV_CONTAINER_CPU: &str = "CONTAINER_CPU";
pub const DEFAULT_CONTAINER_CPU: &str = "0.5"; // you get 1/2 of a cpu, i'm being generous

// TODO: add settings for CPU scheduler
// although, i'm unsure if podman supports userspace containers with different schedulers
pub const ENV_CONTAINER_MEMORY: &str = "CONTAINER_MEMORY";
pub const DEFAULT_CONTAINER_MEMORY: &str = "100m";

pub const ENV_CONTAINER_SWAP: &str = "CONTAINER_SWAP";
pub const DEFAULT_CONTAINER_SWAP: &str = "5m";