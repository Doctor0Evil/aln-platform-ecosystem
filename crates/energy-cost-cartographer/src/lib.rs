// filename: lib.rs
// destination: crates/energy-cost-cartographer/src/
pub mod power_model;

pub use power_model::{
    HardwareProfile, WorkloadProfile, EnergyCostResult,
    compute_power_consumption, compute_ker_efficiency,
};
