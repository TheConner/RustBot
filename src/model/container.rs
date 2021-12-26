
/// Settings for our container
#[derive(Clone)]
pub struct ContainerSettings {
    pub cpu: String,
    pub memory: String,
    pub swap: String
}

pub trait RuntimeSettings {
    fn generate_runtime_flags(&self) -> String;
}

impl RuntimeSettings for ContainerSettings {
    fn generate_runtime_flags(&self) -> String {
        // BUG: when swap is included, we get a OCI runtime error as memory+swap is greater than configured memory
        // fix and re-add swap constraint
        return format!("--cpus={} --memory={}", self.cpu, self.memory);
    }
}