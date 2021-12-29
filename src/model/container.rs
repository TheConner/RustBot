/// Settings for our container
#[derive(Clone)]
pub struct ContainerSettings {
    pub cpu: String,
    pub memory: String,
    pub swap: String,
    pub image: String,
}

pub trait RuntimeSettings {
    fn generate_runtime_flags(&self, is_container: bool) -> String;
}

impl RuntimeSettings for ContainerSettings {
    /// Turns a ContainerSettings instance into a string of CLI args for Podman or Docker
    /// is_container: describes if we are running rustbot in a container
    fn generate_runtime_flags(&self, is_container: bool) -> String {
        // BUG: when swap is included, we get a OCI runtime error as memory+swap is greater than configured memory
        // fix and re-add swap constraint
        // NOTE: podman-in-podman requires cgroups to set resources, which isn't available within nested containers
        // so, admins will have to limit the resources on the outer container themselves
        if is_container {
            return String::from("");
        } else {
            return format!("--cpus={} --memory={}", self.cpu, self.memory);
        }
    }
}
