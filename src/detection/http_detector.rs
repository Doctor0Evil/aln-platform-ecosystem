use regex::Regex;

#[derive(Debug, Clone)]
pub struct DetectedPlatform {
    pub name: String,
    pub source: String,
}

pub struct PlatformDetector {
    poe_re: Regex,
    chatgpt_re: Regex,
    perplexity_re: Regex,
    legacy_re: Regex,
}

impl PlatformDetector {
    pub fn new() -> Self {
        PlatformDetector {
            poe_re: Regex::new(r"PoeClient/v\d+\.\d+").unwrap(),
            chatgpt_re: Regex::new(r"OpenAI::Client/\d+\.\d+").unwrap(),
            perplexity_re: Regex::new(r"PerplexityAI/\d+\.\d+").unwrap(),
            legacy_re: Regex::new(r"Mozilla/\d+").unwrap(),
        }
    }

    pub fn detect_from_headers(
        &self,
        user_agent: &str,
        platform_token: Option<&str>,
    ) -> Option<DetectedPlatform> {
        if let Some(token) = platform_token {
            return Some(DetectedPlatform {
                name: token.to_string(),
                source: "cookie".into(),
            });
        }

        if self.poe_re.is_match(user_agent) {
            return Some(DetectedPlatform {
                name: "poe".into(),
                source: "user_agent".into(),
            });
        }
        if self.chatgpt_re.is_match(user_agent) {
            return Some(DetectedPlatform {
                name: "chatgpt".into(),
                source: "user_agent".into(),
            });
        }
        if self.perplexity_re.is_match(user_agent) {
            return Some(DetectedPlatform {
                name: "perplexity".into(),
                source: "user_agent".into(),
            });
        }
        if self.legacy_re.is_match(user_agent) {
            return Some(DetectedPlatform {
                name: "legacy".into(),
                source: "user_agent".into(),
            });
        }

        None
    }

    pub fn manual_override(
        cli_flag: Option<&str>,
        env_var: Option<&str>,
    ) -> Option<DetectedPlatform> {
        if let Some(p) = cli_flag {
            return Some(DetectedPlatform {
                name: p.to_string(),
                source: "cli_flag".into(),
            });
        }
        if let Some(p) = env_var {
            return Some(DetectedPlatform {
                name: p.to_string(),
                source: "env_var".into(),
            });
        }
        None
    }
}
