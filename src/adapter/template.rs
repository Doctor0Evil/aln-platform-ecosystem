use crate::config::AdapterConfig;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BaseAdapterTemplate {
    pub api_endpoints: HashMap<String, String>,
    pub security_policy: HashMap<String, String>,
}

impl BaseAdapterTemplate {
    pub fn new(platform_name: &str) -> Self {
        let mut endpoints = HashMap::new();
        endpoints.insert(
            "code_execution".into(),
            format!("https://api.{}/v1/execute", platform_name),
        );
        endpoints.insert(
            "asset_upload".into(),
            format!("https://cdn.{}/upload", platform_name),
        );

        let mut security = HashMap::new();
        security.insert("encryption".into(), "AES-256-GCM".into());
        security.insert("hmac".into(), "hmac-sha512".into());

        BaseAdapterTemplate {
            api_endpoints: endpoints,
            security_policy: security,
        }
    }

    pub fn adapt_with_config(
        &self,
        platform_name: &str,
        platform_config: HashMap<String, String>,
    ) -> AdapterConfig {
        let mut merged_endpoints = self.api_endpoints.clone();
        for (k, v) in platform_config.iter() {
            if k.starts_with("endpoint.") {
                let key = k.trim_start_matches("endpoint.").to_string();
                merged_endpoints.insert(key, v.clone());
            }
        }

        let mut merged_security = self.security_policy.clone();
        for (k, v) in platform_config.iter() {
            if k.starts_with("security.") {
                let key = k.trim_start_matches("security.").to_string();
                merged_security.insert(key, v.clone());
            }
        }

        AdapterConfig {
            name: platform_name.to_string(),
            api_endpoints: merged_endpoints,
            security: merged_security,
        }
    }
}
