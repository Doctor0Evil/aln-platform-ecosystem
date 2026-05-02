```
crates/energy-cost-cartographer/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Public API: place_workload(), compute_eta()
│   ├── power_model.rs      # P_watts(w,h) implementation with calibration
│   ├── ker_gain.rs         # ΔKER(w,r,h) with risk propagation kernels
│   ├── constraints.rs      # Corridor admissibility checks (𝒜_j, 𝒩)
│   ├── optimizer.rs        # Branch-and-bound placement solver
│   └── telemetry.rs        # Hardware metric ingestion and normalization
├── tests/
│   ├── placement_monotone.rs  # Property tests for monotonicity
│   └── corridor_tightening.rs # Verify "tightest band wins"
└── benches/
    └── placement_latency.rs   # Real-time performance validation
    ```
