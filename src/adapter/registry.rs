use crate::config::{AdapterConfig, PlatformRegistry};
use log::{info, warn};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AdapterInstance {
    pub name: String,
    pub endpoints: HashMap<String, String>,
    pub security: HashMap<String, String>,
}

#[derive(Debug)]
pub struct AdapterRegistry {
    pub registry: HashMap<String, AdapterInstance>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        AdapterRegistry {
            registry: HashMap::new(),
        }
    }

    pub fn load_adapter(
        &mut self,
        platform_name: &str,
        platform_registry: &PlatformRegistry,
    ) -> Option<AdapterInstance> {
        if !platform_registry.exists(platform_name) {
            warn!(
                "Using generic adapter for unknown platform: {}",
                platform_name
            );
            let generic = AdapterInstance {
                name: "generic".into(),
                endpoints: HashMap::new(),
                security: HashMap::new(),
            };
            self.registry.insert("generic".into(), generic.clone());
            return Some(generic);
        }

        let config = AdapterConfig {
            name: platform_name.to_string(),
            api_endpoints: HashMap::from([
                (
                    "code".into(),
                    format!("https://{}/api/v2", platform_name),
                ),
                (
                    "assets".into(),
                    format!("https://{}/storage", platform_name),
                ),
            ]),
            security: HashMap::from([
                ("auth".into(), "oauth2".into()),
                ("encryption".into(), "chacha20-poly1305".into()),
            ]),
        };

        let instance = AdapterInstance {
            name: config.name.clone(),
            endpoints: config.api_endpoints.clone(),
            security: config.security.clone(),
        };

        info!("Registered support for {}", platform_name);
        self.registry
            .insert(platform_name.to_string(), instance.clone());
        Some(instance)
    }
}
