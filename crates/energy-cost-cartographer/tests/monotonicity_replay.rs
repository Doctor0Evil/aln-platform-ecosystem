// filename: monotonicity_replay.rs
// destination: crates/energy-cost-cartographer/tests/
use energy_cost_cartographer::{HardwareProfile, WorkloadProfile, compute_ker_efficiency};

/// Property test: Worsening hardware efficiency or increasing workload intensity
/// must never increase KER-efficiency (eta). eta_new <= eta_old.
#[test]
fn verify_monotonic_placement_efficiency() {
    let baseline_hw = HardwareProfile {
        pue: 1.15,
        cpu_eff_flops_per_watt: 150.0,
        mem_eff_gb_per_watt: 80.0,
        net_eff_mb_per_watt: 250.0,
        idle_power_watts: 3.0,
        grid_carbon_kg_per_kwh: 0.42,
    };

    let baseline_wl = WorkloadProfile {
        cpu_ops: 2000.0,
        mem_bw_gb: 5.0,
        net_io_mb: 100.0,
    };

    let delta_ker = 0.85;
    let baseline_eta = compute_ker_efficiency(&baseline_hw, &baseline_wl, delta_ker).unwrap().ker_efficiency;

    // Perturbations representing "worse" conditions
    let degraded_hw = HardwareProfile {
        pue: 1.30,
        cpu_eff_flops_per_watt: 120.0,
        mem_eff_gb_per_watt: 60.0,
        net_eff_mb_per_watt: 200.0,
        idle_power_watts: 5.0,
        grid_carbon_kg_per_kwh: 0.55,
    };

    let heavier_wl = WorkloadProfile {
        cpu_ops: 2000.0 * 1.5,
        mem_bw_gb: 5.0 * 1.5,
        net_io_mb: 100.0 * 1.5,
    };

    let degraded_eta = compute_ker_efficiency(&degraded_hw, &heavier_wl, delta_ker).unwrap().ker_efficiency;

    // Monotonicity invariant: efficiency must not improve under worse inputs
    assert!(
        degraded_eta <= baseline_eta,
        "Placement monotonicity violated: degraded inputs yielded higher efficiency. baseline={}, degraded={}",
        baseline_eta,
        degraded_eta
    );
}

/// Replay simulation: Verify that higher risk planes (rcarbon, rbiodiv) reduce delta_KER,
/// which in turn reduces eta.
#[test]
fn verify_risk_sensitive_eta_decay() {
    let hw = HardwareProfile { pue: 1.1, cpu_eff_flops_per_watt: 200.0, mem_eff_gb_per_watt: 100.0, net_eff_mb_per_watt: 300.0, idle_power_watts: 2.0, grid_carbon_kg_per_kwh: 0.3 };
    let wl = WorkloadProfile { cpu_ops: 3000.0, mem_bw_gb: 8.0, net_io_mb: 150.0 };

    let safe_delta_ker = 0.90;
    let risky_delta_ker = 0.65; // Penalized by non-offsettable plane crossings

    let safe_eta = compute_ker_efficiency(&hw, &wl, safe_delta_ker).unwrap().ker_efficiency;
    let risky_eta = compute_ker_efficiency(&hw, &wl, risky_delta_ker).unwrap().ker_efficiency;

    assert!(
        risky_eta <= safe_eta,
        "Non-offsettable risk penalty failed: higher risk yielded better efficiency."
    );
}
