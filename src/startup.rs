//! src/startup.rs
use crate::configuration::Settings;
use anyhow::Result;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_memorydb::Client;
use aws_sdk_memorydb::operation::create_cluster::CreateClusterOutput;

pub struct Application {
    pub client: Client,
    pub response: CreateClusterOutput,
}

impl Application {
    #[tracing::instrument()]
    pub async fn build(configuration: Settings) -> Result<Self> {
        let client_config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(configuration.region))
            .load()
            .await;

        let client = Client::new(&client_config);

        let subnet_group = client
            .create_subnet_group()
            .set_subnet_group_name(Some(configuration.cluster_settings.subnet_group_name))
            .set_subnet_ids(Some(configuration.cluster_settings.subnet_ids))
            .send()
            .await?;

        let subnet_group_name = subnet_group.subnet_group.unwrap().name;

        let response = client
            .create_cluster()
            .cluster_name(configuration.cluster_settings.name)          
            .description(configuration.cluster_settings.description)    
            .set_subnet_group_name(subnet_group_name)                       
            .node_type(configuration.cluster_settings.node_type)        
            .acl_name(configuration.cluster_settings.acl_name)          
            .num_shards(configuration.cluster_settings.num_shards)      
            .num_replicas_per_shard(configuration.cluster_settings.num_replicas_per_shard)
            .send()
            .await?;

        Ok(Self { client, response })
    }
}
