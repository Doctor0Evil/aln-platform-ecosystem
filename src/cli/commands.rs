use crate::adapter::{AdapterFailureHandler, AdapterRegistry};
use crate::api::{CodeExecutionRequest, UnifiedApi};
use crate::config::{PlatformRegistry, SecurityConfig, SecurityPolicy};
use crate::detection::PlatformDetector;
use crate::security::{CodeContext, CodeSanitizer};
use clap::{Parser, Subcommand};
use log::{info, warn};
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about = "ALN Platform Ecosystem CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Expand platform support for new platforms (mirrors @ACTION expand_platform_support)
    Expand {
        #[arg(long = "new-platforms", value_delimiter = ',')]
        new_platforms: Vec<String>,
    },

    /// Reload all adapters (mirrors `aln adapter.reload --all`)
    ReloadAdapters {},

    /// Run a platform compatibility check
    Check {
        #[arg(long)]
        platform: String,
    },

    /// Execute code via the cross-platform API
    Exec {
        #[arg(long)]
        platform: String,
        #[arg(long)]
        language: String,
        #[arg(long)]
        code: String,
    },
}

impl Cli {
    pub fn execute(self) {
        let mut registry = PlatformRegistry::from_aln_spec();
        let security_config = SecurityConfig {
            all_platforms: SecurityPolicy {
                xss_protection: "DOMPurify@v3.0.0".into(),
                sql_injection: "sqlmap@strict".into(),
                command_injection: "bash_escape@v2".into(),
                path_traversal: "pathval@strict".into(),
            },
            platform_specific: Default::default(),
        };
        let sanitizer = CodeSanitizer::new(security_config);
        let api = UnifiedApi::new(sanitizer);
        let mut adapters = AdapterRegistry::new();
        let failure_handler = AdapterFailureHandler;
        let detector = PlatformDetector::new();

        match self.command {
            Commands::Expand { new_platforms } => {
                registry.add_platforms(&new_platforms);
                for p in &new_platforms {
                    adapters.load_adapter(p, &registry);
                    info!("Registered support for {}", p);
                }
            }
            Commands::ReloadAdapters {} => {
                warn!("Reloading all adapters (noop placeholder)");
            }
            Commands::Check { platform } => {
                let (compatible, feature_set) = registry.compatibility_check(&platform);
                println!("Platform: {}", platform);
                println!("Compatible: {}", compatible);
                println!("Features: {:?}", feature_set.features);
                println!("Restrictions: {:?}", feature_set.restrictions);
            }
            Commands::Exec {
                platform,
                language,
                code,
            } => {
                let ua = env::var("HTTP_USER_AGENT").unwrap_or_default();
                let detected = detector.detect_from_headers(&ua, None);
                if let Some(d) = detected {
                    info!("Detected platform={} via {}", d.name, d.source);
                }

                let req = CodeExecutionRequest {
                    platform: platform.clone(),
                    language: language.clone(),
                    code: code.clone(),
                };
                match api.execute_code(req) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => {
                        let _ = failure_handler.on_failure(1);
                        eprintln!("Execution failed: {}", e);
                    }
                }
            }
        }
    }
}
