use tokio::sync::RwLock;

struct PluginRuntime {
    instances: Vec<PluginInstance>,
}

struct PluginInstance {
    stdout: RwLock<Vec<u8>>,
}

impl PluginInstance {}
