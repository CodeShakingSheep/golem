
#[cfg(test)]
mod tests {
    use golem_worker_executor_base::services::golem_config::{
        GolemConfig, ShardManagerServiceConfig,
    };

    #[test]
    pub fn config_is_loadable() {
        // The following settings are always coming through environment variables:
        std::env::set_var("GOLEM__REDIS__HOST", "localhost");
        std::env::set_var("GOLEM__REDIS__PORT", "1234");
        std::env::set_var("GOLEM__REDIS__DATABASE", "1");
        std::env::set_var("GOLEM__TEMPLATE_SERVICE__CONFIG__HOST", "localhost");
        std::env::set_var("GOLEM__TEMPLATE_SERVICE__CONFIG__PORT", "1234");
        std::env::set_var("GOLEM__TEMPLATE_SERVICE__CONFIG__ACCESS_TOKEN", "token");
        std::env::set_var(
            "GOLEM__COMPILED_TEMPLATE_SERVICE__CONFIG__REGION",
            "us-east-1",
        );
        std::env::set_var(
            "GOLEM__COMPILED_TEMPLATE_SERVICE__CONFIG__BUCKET",
            "golem-compiled-components",
        );
        std::env::set_var(
            "GOLEM__COMPILED_TEMPLATE_SERVICE__CONFIG__OBJECT_PREFIX",
            "",
        );
        std::env::set_var("GOLEM__BLOB_STORE_SERVICE__CONFIG__REGION", "us-east-1");
        std::env::set_var(
            "GOLEM__BLOB_STORE_SERVICE__BUCKET",
            "golem-compiled-components",
        );
        std::env::set_var("GOLEM__BLOB_STORE_SERVICE__CONFIG__OBJECT_PREFIX", "");
        std::env::set_var("GOLEM__SHARD_MANAGER_SERVICE__CONFIG__HOST", "localhost");
        std::env::set_var("GOLEM__SHARD_MANAGER_SERVICE__CONFIG__PORT", "4567");
        std::env::set_var("GOLEM__PORT", "1234");
        std::env::set_var("GOLEM__HTTP_PORT", "1235");
        std::env::set_var("GOLEM__ENABLE_JSON_LOG", "true");

        // The rest can be loaded from the toml
        let golem_config = GolemConfig::new();

        let shard_manager_grpc_port = match &golem_config.shard_manager_service {
            ShardManagerServiceConfig::Grpc(config) => config.port,
            _ => panic!("Expected shard manager service to be grpc"),
        };
        assert_eq!(shard_manager_grpc_port, 4567);
    }
}
