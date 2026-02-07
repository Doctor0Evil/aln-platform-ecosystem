pub mod registry;
pub mod template;
pub mod self_healing;

pub use registry::{AdapterInstance, AdapterRegistry};
pub use template::BaseAdapterTemplate;
pub use self_healing::{AdapterFailureHandler, SelfHealingResult};
