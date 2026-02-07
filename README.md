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
