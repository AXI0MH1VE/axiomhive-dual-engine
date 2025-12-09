use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalContext {
    pub logical_time: u64,
    pub rng_seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadSpec {
    pub module_path: String,
    pub entrypoint: String,
    pub input_json: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeeOutput {
    pub stdout: String,
    pub stderr: String,
    pub result_json: serde_json::Value,
    pub digest_sha384: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceBundle {
    pub dee_output: DeeOutput,
    pub build_provenance: serde_json::Value,
    pub sbom: serde_json::Value,
    pub pq_signatures: Vec<PqSignature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqSignature {
    pub role: String,
    pub algorithm: String,
    pub public_key_id: String,
    pub signature_b64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyInvariant {
    pub name: String,
    pub status: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyEnvelope {
    pub invariants: Vec<SafetyInvariant>,
    pub max_blast_radius: f64,
    pub rollback_possible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanaryResult {
    pub max_blast_radius: f64,
    pub rollback_possible: bool,
    pub digest_sha384: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtInput {
    pub adr_hash: String,
    pub safety_envelope: SafetyEnvelope,
    pub shadow_digest: String,
    pub canary_result: CanaryResult,
    pub attestations: Vec<PqSignature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtDecision {
    pub permit: bool,
    pub reason: String,
}
