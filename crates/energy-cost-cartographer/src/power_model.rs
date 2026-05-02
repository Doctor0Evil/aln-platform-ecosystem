// filename: power_model.rs
// destination: crates/energy-cost-cartographer/src/
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct HardwareProfile {
    pub pue: f64,
    pub cpu_eff_flops_per_watt: f64,
    pub mem_eff_gb_per_watt: f64,
    pub net_eff_mb_per_watt: f64,
    pub idle_power_watts: f64,
    pub grid_carbon_kg_per_kwh: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkloadProfile {
    pub cpu_ops: f64,
    pub mem_bw_gb: f64,
    pub net_io_mb: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnergyCostResult {
    pub power_watts: f64,
    pub carbon_kg: f64,
    pub ker_efficiency: f64,
}

#[derive(Error, Debug)]
pub enum CartographerError {
    #[error("hardware efficiency values must be strictly positive")]
    NonPositiveEfficiency,
    #[error("PUE must be >= 1.0")]
    InvalidPue,
}

/// Computes raw power consumption: P = PUE * (CPU_ops/eps_cpu + MEM/eps_mem + NET/eps_net) + P_idle
pub fn compute_power_consumption(hw: &HardwareProfile, wl: &WorkloadProfile) -> Result<f64, CartographerError> {
    if hw.pue < 1.0 {
        return Err(CartographerError::InvalidPue);
    }
    if hw.cpu_eff_flops_per_watt <= 0.0 || hw.mem_eff_gb_per_watt <= 0.0 || hw.net_eff_mb_per_watt <= 0.0 {
        return Err(CartographerError::NonPositiveEfficiency);
    }

    let compute_watts = wl.cpu_ops / hw.cpu_eff_flops_per_watt
        + wl.mem_bw_gb / hw.mem_eff_gb_per_watt
        + wl.net_io_mb / hw.net_eff_mb_per_watt;

    let total = hw.pue * compute_watts + hw.idle_power_watts;
    Ok(total.max(0.0))
}

/// Computes carbon footprint and KER-efficiency metric:
/// eta = delta_KER / P_watts
pub fn compute_ker_efficiency(
    hw: &HardwareProfile,
    wl: &WorkloadProfile,
    delta_ker: f64,
) -> Result<EnergyCostResult, CartographerError> {
    let p = compute_power_consumption(hw, wl)?;
    let energy_kwh = (p * 1.0 / 3600.0) / 1000.0; // Convert W -> kWh for 1s window
    let carbon = energy_kwh * hw.grid_carbon_kg_per_kwh;

    let eta = if p <= 0.0001 { f64::MAX } else { delta_ker.max(0.0) / p };

    Ok(EnergyCostResult {
        power_watts: p,
        carbon_kg: carbon,
        ker_efficiency: eta,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baseline_consumption() {
        let hw = HardwareProfile { pue: 1.2, cpu_eff_flops_per_watt: 100.0, mem_eff_gb_per_watt: 50.0, net_eff_mb_per_watt: 200.0, idle_power_watts: 5.0, grid_carbon_kg_per_kwh: 0.4 };
        let wl = WorkloadProfile { cpu_ops: 1000.0, mem_bw_gb: 2.0, net_io_mb: 50.0 };
        let p = compute_power_consumption(&hw, &wl).unwrap();
        assert!((p - 20.0).abs() < 1e-6);
    }

    #[test]
    fn test_ker_efficiency_calculation() {
        let hw = HardwareProfile { pue: 1.1, cpu_eff_flops_per_watt: 200.0, mem_eff_gb_per_watt: 100.0, net_eff_mb_per_watt: 300.0, idle_power_watts: 2.0, grid_carbon_kg_per_kwh: 0.35 };
        let wl = WorkloadProfile { cpu_ops: 5000.0, mem_bw_gb: 10.0, net_io_mb: 200.0 };
        let res = compute_ker_efficiency(&hw, &wl, 0.95).unwrap();
        assert!(res.ker_efficiency > 0.0);
    }
}
