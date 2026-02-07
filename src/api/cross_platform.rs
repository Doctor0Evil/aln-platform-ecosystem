use crate::security::{CodeContext, CodeSanitizer};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionRequest {
    pub platform: String,
    pub language: String,
    pub code: String,
}

pub struct UnifiedApi {
    sanitizer: CodeSanitizer,
}

impl UnifiedApi {
    pub fn new(sanitizer: CodeSanitizer) -> Self {
        UnifiedApi { sanitizer }
    }

    pub fn execute_code(&self, req: CodeExecutionRequest) -> anyhow::Result<String> {
        let ctx = CodeContext {
            platform: req.platform.clone(),
            language: req.language.clone(),
            raw_code: req.code.clone(),
        };

        let sanitized = self.sanitizer.sanitize(&ctx);
        info!(
            "Executing sanitized code on platform={} language={}",
            req.platform, req.language
        );

        Ok(format!("Executed code for platform {}", req.platform))
    }

    pub fn upload_asset(
        &self,
        platform: &str,
        name: &str,
        size_bytes: u64,
        content_type: &str,
    ) -> anyhow::Result<String> {
        if size_bytes > 10 * 1024 * 1024 {
            anyhow::bail!("Asset too large, max 10MB");
        }

        let allowed = ["png", "jpg", "json", "aln", "md"];
        let ok = allowed
            .iter()
            .any(|ext| name.to_lowercase().ends_with(ext));
        if !ok {
            anyhow::bail!("Unsupported asset format");
        }

        info!(
            "Uploading asset {} ({} bytes, {}) to platform={}",
            name, size_bytes, content_type, platform
        );

        Ok(format!(
            "https://cdn.{}/assets/{}",
            platform.to_lowercase(),
            name
        ))
    }
}
