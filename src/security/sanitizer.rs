use crate::config::SecurityConfig;
use log::debug;

#[derive(Debug, Clone)]
pub struct CodeContext {
    pub platform: String,
    pub language: String,
    pub raw_code: String,
}

pub struct CodeSanitizer {
    security_config: SecurityConfig,
}

impl CodeSanitizer {
    pub fn new(security_config: SecurityConfig) -> Self {
        CodeSanitizer { security_config }
    }

    pub fn sanitize(&self, ctx: &CodeContext) -> String {
        debug!(
            "Sanitizing code for platform={} language={}",
            ctx.platform, ctx.language
        );

        let _policy = &self.security_config.all_platforms;

        ctx.raw_code.clone()
    }
}
