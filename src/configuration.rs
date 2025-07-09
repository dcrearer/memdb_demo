//! src/configuration.rs
use config;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub region: String,
    pub cluster_settings: ClusterSettings,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ClusterSettings {
    pub name: String,
    pub description: String,
    pub node_type: String,
    pub acl_name: String,
    pub num_shards: i32,
    pub num_replicas_per_shard: i32,
    pub subnet_group_name: String,
    pub subnet_ids: Vec<String>,
}

#[tracing::instrument()]
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current working directory");
    let configuration_directory = base_path.join("configuration");
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("config.yaml"),
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
