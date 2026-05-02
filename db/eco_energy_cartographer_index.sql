-- filename: eco_energy_cartographer_index.sql
-- destination: db/eco_energy_cartographer_index.sql
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS energy_hardware_profiles (
    hardware_class TEXT PRIMARY KEY CHECK (hardware_class IN ('EDGE_IOT','FOG_NODE','CLOUD_VM','HPC_CLUSTER')),
    pue REAL NOT NULL CHECK (pue >= 1.0),
    cpu_eff_flops_per_watt REAL NOT NULL CHECK (cpu_eff_flops_per_watt > 0),
    mem_eff_gb_per_watt REAL NOT NULL CHECK (mem_eff_gb_per_watt > 0),
    net_eff_mb_per_watt REAL NOT NULL CHECK (net_eff_mb_per_watt > 0),
    idle_power_watts REAL NOT NULL CHECK (idle_power_watts >= 0),
    grid_carbon_kg_per_kwh REAL NOT NULL CHECK (grid_carbon_kg_per_kwh >= 0),
    lyapunov_weight REAL NOT NULL CHECK (lyapunov_weight >= 0),
    non_offsettable INTEGER NOT NULL DEFAULT 1 CHECK (non_offsettable IN (0,1)),
    last_calibrated_utc TEXT
);

CREATE TABLE IF NOT EXISTS energy_placement_decisions (
    decision_id INTEGER PRIMARY KEY AUTOINCREMENT,
    workload_id TEXT NOT NULL,
    hardware_class TEXT NOT NULL REFERENCES energy_hardware_profiles(hardware_class),
    delta_ker REAL NOT NULL CHECK (delta_ker BETWEEN 0.0 AND 1.0),
    power_watts REAL NOT NULL,
    eta_score REAL NOT NULL CHECK (eta_score >= 0.0),
    lane TEXT NOT NULL CHECK (lane IN ('RESEARCH','PILOT','PROD')),
    kerdeployable INTEGER NOT NULL CHECK (kerdeployable IN (0,1)),
    evidence_hex TEXT,
    created_utc TEXT DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);

-- Seed baseline profiles (Phoenix/CAP region defaults)
INSERT OR IGNORE INTO energy_hardware_profiles VALUES
('FOG_NODE', 1.25, 180.0, 90.0, 280.0, 4.5, 0.38, 1.0, 1, '2026-01-01T00:00:00Z'),
('CLOUD_VM', 1.15, 320.0, 140.0, 450.0, 2.0, 0.28, 0.9, 1, '2026-01-01T00:00:00Z');
