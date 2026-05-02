# Energy-Cost Cartographer Wiring Guide

## 1. Grammar Integration
- `EnergyCostField2026v1.aln` defines the hardware/workload parameter space.
- `PlaneWeightsShard2026v1.aln` pins `energy` as a Lyapunov channel with `non_offsettable = true`.
- `r_energy` is now a first-class coordinate in `ecosafety.riskvector.v2`. It contributes to:
  \[
  V_t^{\text{full}} = \sum_{j \in \text{physical}} w_j r_j^2 + w_{\text{energy}} r_{\text{energy}}^2 + w_{\text{calib}} r_{\text{calib}}^2 + w_{\sigma} r_{\sigma}^2
  \]

## 2. KER-Efficiency Mapping
- The crate exposes `compute_ker_efficiency(hw, wl, delta_ker)` returning `eta = delta_KER / P_watts`.
- `delta_KER` is already penalized when non-offsettable planes (`rcarbon`, `rbiodiv`, `r_energy`) cross gold/hard corridors.
- Placement optimizer queries `eta` and filters by `lane` and `kerdeployable` gates.

## 3. CI Enforcement
- Run `cargo test --lib` to verify baseline math.
- Run `cargo test --test monotonicity_replay` to assert:
  \[
  \forall \Delta \text{input} \ge 0,\quad \eta_{\text{new}} \le \eta_{\text{old}}
  \]
- ALN linting stage validates `EnergyCostField2026v1.aln` invariants (`pue_bounds`, `lyapunov_weights_positive`).

## 4. Agent Discovery
- SQLite table `energy_placement_decisions` logs all placement evaluations with `eta_score`, `power_watts`, and `kerdeployable`.
- Agents query:
  ```sql
  SELECT hardware_class, power_watts, eta_score FROM energy_placement_decisions
  WHERE lane = 'PROD' AND kerdeployable = 1 ORDER BY eta_score DESC LIMIT 10;
