use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    pub name: String,
    pub api_endpoints: HashMap<String, String>,
    pub security: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformFeatureSet {
    pub features: Vec<String>,
    pub restrictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub xss_protection: String,
    pub sql_injection: String,
    pub command_injection: String,
    pub path_traversal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub all_platforms: SecurityPolicy,
    pub platform_specific: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Default)]
pub struct PlatformRegistry {
    core_platforms: HashSet<String>,
    legacy_systems: HashSet<String>,
    emerging_platforms: HashSet<String>,
    adapters: HashMap<String, AdapterConfig>,
    features: HashMap<String, PlatformFeatureSet>,
}

impl PlatformRegistry {
    pub fn from_aln_spec() -> Self {
        let core = vec![
            "poe", "perplexity", "chatgpt", "claude", "anthropic", "huggingface",
            "bing_ai", "socratic", "llama2", "dolly", "google_bard", "meta_lamda",
            "aws_titan", "azure_copilot", "deepseek", "nlp_cloud", "indico",
            "open_assistant", "manticore", "mothership", "cohere", "jina_ai",
            "replicate", "valence", "lattice", "cerebra", "synapse", "neural",
            "synth", "quantum", "neuraflux", "mindflow", "brain", "neurosynth",
            "neuralspark", "cortex", "neurocore", "synaptik", "neuralink",
            "neuralspace", "cerebrum", "neuroflow", "synapseai", "neuraltide",
            "neuralsurge", "cerebro", "synthmind", "quantumflow", "neuralflux",
            "neurogrid", "synaptics", "cerebrox", "synapseflow", "quantumcore",
            "neuralwave", "synthcore", "cerebrumx", "quantumflux", "neuralmatrix",
            "synapticsx", "cerebroflow", "neurosynthx", "synthwave", "quantummind",
            "neuralcore", "synapseflux", "neurogridx", "synthcorex", "quantumfluxx",
            "neuralmatrixx", "synapticspro", "cerebroplus", "neurosynthpro",
            "synthwavepro", "quantummindpro", "neuralcoreplus", "synapsefluxpro",
            "neurogridpro", "synthcorepro",
        ];

        let legacy = vec![
            "irc", "xmpp", "telegram", "slack", "discord", "matrix", "whatsapp",
            "signal", "email", "sms", "sip", "jabber", "hangouts", "skype",
            "teams", "reddit", "twitter", "linkedin",
        ];

        let emerging = vec![
            "neuraos", "brainwave", "synthmind2", "quantumflux2",
            "neuralwave2", "synapseflux2", "neurocore2", "cerebro2",
        ];

        PlatformRegistry {
            core_platforms: core.into_iter().map(|s| s.to_string()).collect(),
            legacy_systems: legacy.into_iter().map(|s| s.to_string()).collect(),
            emerging_platforms: emerging.into_iter().map(|s| s.to_string()).collect(),
            adapters: HashMap::new(),
            features: HashMap::new(),
        }
    }

    pub fn register_adapter(&mut self, adapter: AdapterConfig) {
        self.adapters.insert(adapter.name.clone(), adapter);
    }

    pub fn exists(&self, name: &str) -> bool {
        self.core_platforms.contains(name)
            || self.legacy_systems.contains(name)
            || self.emerging_platforms.contains(name)
            || self.adapters.contains_key(name)
    }

    pub fn add_platforms(&mut self, new_platforms: &[String]) {
        for p in new_platforms {
            self.emerging_platforms.insert(p.clone());
        }
    }

    pub fn compatibility_check(
        &self,
        target_platform: &str,
    ) -> (bool, PlatformFeatureSet) {
        let compatible = self.exists(target_platform);
        let feature_set = self
            .features
            .get(target_platform)
            .cloned()
            .unwrap_or(PlatformFeatureSet {
                features: vec!["execute_code".into(), "upload_asset".into()],
                restrictions: vec!["rate_limit: default".into()],
            });
        (compatible, feature_set)
    }

    pub fn load_platform_spec<P: AsRef<Path>>(
        &mut self,
        name: &str,
        path: P,
    ) -> anyhow::Result<()> {
        let raw = fs::read_to_string(path)?;
        let adapter: AdapterConfig = serde_json::from_str(&raw)?;
        self.register_adapter(adapter);
        Ok(())
    }
}
