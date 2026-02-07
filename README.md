# ALN Platform Ecosystem

This repository implements a production-ready ALN-driven platform adapter ecosystem for AI platforms, legacy systems, and emerging runtimes.

It provides:

- Dynamic **platform** detection from HTTP headers and manual overrides.
- A unified adapter registry and template system.
- Cross-platform code execution and asset upload APIs.
- Security sanitation policies mirroring the ALN specification.
- Self-healing and rollback mechanisms for adapter failures.
- CLI commands to expand platform support and reload adapters.

The `aln/` directory contains the canonical ALN specs that drive the Rust implementation under `src/`.


aln-platform-ecosystem/
├─ Cargo.toml
├─ aln-platform-ecosystem.sln                     # optional for C++/Java tooling
├─ README.md
├─ aln/
│  ├─ platform/
│  │  ├─ ecosystem.aln                           # Core ALN spec derived from prompt
│  │  ├─ security_policies.aln
│  │  └─ adapters.aln
│  └─ workflows/
│     └─ expansion_flow.aln
├─ src/
│  ├─ lib.rs
│  ├─ main.rs
│  ├─ config/
│  │  ├─mod.rs
│  │  └─ platform_registry.rs
│  ├─ detection/
│  │  ├─ mod.rs
│  │  └─ http_detector.rs
│  ├─ adapter/
│  │  ├─ mod.rs
│  │  ├─ registry.rs
│  │  ├─ template.rs
│  │  └─ self_healing.rs
│  ├─ security/
│  │  ├─ mod.rs
│  │  └─ sanitizer.rs
│  ├─ api/
│  │  ├─ mod.rs
│  │  └─ cross_platform.rs
│  └─ cli/
│     ├─ mod.rs
│     └─ commands.rs
├─ config/
│  ├─ platforms/
│  │  ├─ poe.json
│  │  ├─ chatgpt.json
│  │  ├─ perplexity.json
│  │  ├─ neuraos.json
│  │  ├─ quantumflux2.json
│  │  ├─ cerebroplus.json
│  │  └─ synthwavepro.json
│  └─ security.json
└─ tools/
   └─ adapter-reload.sh
