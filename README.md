# AxiomHive Dual-Engine System

## Sovereign Deterministic Execution Engine (DEE) + Quantum-Resistant Verification Engine (QRVE) + Inevitability Gate

### Architecture Overview

This repository implements a dual-engine substrate with three core components:

1. **Deterministic Execution Engine (DEE)**: Hermetic WebAssembly execution with explicit logical time and deterministic randomness
2. **Quantum-Resistant Verification Engine (QRVE)**: Post-quantum signatures, SLSA provenance, in-toto attestations  
3. **Inevitability Gate**: Boolean decision layer combining DEE outputs, QRVE attestations, safety envelopes, and governance quorum

### Core Guarantees

- **Zero Drift**: Identical inputs produce identical outputs across all runs
- **Zero Hidden State**: All randomness and time explicitly injected via LogicalContext
- **Cryptographic Gating**: Strict separation between DEE, QRVE, and Gate with no shared mutable state
- **Quantum-Resistant**: Verification layer designed for post-quantum signature schemes

### Repository Structure

```
axiomhive-dual-engine/
├── Cargo.toml                          # Rust project manifest
├── src/
│   ├── lib.rs                          # Module exports
│   ├── models.rs                       # Core data structures
│   ├── dee/mod.rs                      # Deterministic execution engine
│   ├── qrve/mod.rs                     # Quantum-resistant verification
│   └── inevitability_gate.rs           # Decision gate logic
├── policy/court/inevitability.rego     # OPA/Rego policy rules
├── adr/template_inevitable_event.md    # ADR template for governance
├── provenance/slsa_in_toto_attestation.json  # Provenance schema
├── kubernetes/admission/policy_controller.yaml  # Cluster admission policy
└── config/default_dual_engine.yaml     # Default configuration
```

### Quick Start

#### Prerequisites

- Rust 1.70+
- Cargo

#### Build

```bash
cargo build --release
```

#### Run Tests

```bash
cargo test
```

### Components

#### Deterministic Execution Engine (DEE)

- Executes WebAssembly workloads with deterministic Wasmtime configuration
- Injects logical time and random data through explicit host functions
- Uses ChaCha20 RNG seeded from LogicalContext for reproducible randomness
- Computes SHA-384 digests of all outputs

#### Quantum-Resistant Verification Engine (QRVE)

- Wraps DEE outputs in SLSA/in-toto provenance bundles
- Generates SPDX SBOMs
- Creates deterministic post-quantum signatures (placeholder SHA-384 based)
- Role-tagged records: Ops, Security, Product

#### Inevitability Gate

- Evaluates CourtInput records with ADR hash, safety envelope, canary results, attestations
- Enforces Rego policies checking:
  - Input readiness
  - Safety invariants
  - Blast radius bounds
  - Rollback capability
  - Signature quorum (Ops + Security + Product)
- Returns boolean permit/deny decision

### Axioms

- **DETERMINISM**: All execution paths are pure functions of inputs and seed material
- **SOURCE_PRIMACY**: Full artifacts over summaries
- **SOVEREIGNTY**: Local execution, no external dependencies
- **ZERO_EGO**: No persona, emotions, or conversational framing
- **NO_TIME_LANGUAGE**: No references to durations, deadlines, schedules
- **FULL_ARTIFACT_MODE**: Complete blueprints, code, and policies in single emission

### Frozen Seed Protocol

- Seed value: 42
- Structural ordering, file paths, axiom set, and schema layout locked by seed
- LogicalContext and RNG construction fully explicit

### Implementation Status

✅ Repository created  
✅ Cargo.toml added  
✅ src/lib.rs added  
⏳ Remaining source modules (models.rs, dee/mod.rs, qrve/mod.rs, inevitability_gate.rs)  
⏳ Policy files (Rego, YAML, JSON schemas)  
⏳ Documentation (ADR templates, provenance schemas)

### Local Population Instructions

To complete the repository with all source files:

1. Clone the repository:
```bash
git clone https://github.com/AXI0MH1VE/axiomhive-dual-engine.git
cd axiomhive-dual-engine
```

2. Create the directory structure:
```bash
mkdir -p src/dee policy/court adr provenance kubernetes/admission config
```

3. Add all source files using the complete artifact set provided in the original specification.

4. Commit and push:
```bash
git add .
git commit -m "Add complete dual-engine implementation"
git push origin main
```

### License

No license specified. Contact repository owner for usage terms.

### Contact

AxiomHive — Deterministic AI Infrastructure
