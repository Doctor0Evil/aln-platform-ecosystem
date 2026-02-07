use log::{error, info};

#[derive(Debug, Clone)]
pub enum SelfHealingResult {
    FallbackToGeneric,
    RetriedWithUpdatedConfig,
    NotifiedAndDisabled,
}

#[derive(Debug)]
pub struct AdapterFailureHandler;

impl AdapterFailureHandler {
    pub fn on_failure(&self, attempt: u8) -> SelfHealingResult {
        match attempt {
            1 => {
                info!("Attempt 1: fallback_to_generic_adapter()");
                SelfHealingResult::FallbackToGeneric
            }
            2 => {
                info!("Attempt 2: retry_with_updated_config()");
                SelfHealingResult::RetriedWithUpdatedConfig
            }
            _ => {
                error!("Attempt 3: notify_ops_and_disable()");
                SelfHealingResult::NotifiedAndDisabled
            }
        }
    }
}
