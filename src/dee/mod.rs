use crate::models::{DeeOutput, LogicalContext, WorkloadSpec};
use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};
use sha2::{Digest, Sha384};
use wasmtime::*;

pub trait DeterministicHost {
    fn logical_context(&self) -> LogicalContext;
    fn fill_rng(&mut self, buf: &mut [u8]);
}

pub struct DefaultDeterministicHost {
    ctx: LogicalContext,
    rng: ChaCha20Rng,
}

impl DefaultDeterministicHost {
    pub fn new(logical_time: u64, rng_seed: u64) -> Self {
        let mut seed_bytes = [0u8; 32];
        seed_bytes[0..8].copy_from_slice(&logical_time.to_le_bytes());
        seed_bytes[8..16].copy_from_slice(&rng_seed.to_le_bytes());
        let rng = ChaCha20Rng::from_seed(seed_bytes);
        Self {
            ctx: LogicalContext {
                logical_time,
                rng_seed,
            },
            rng,
        }
    }
}

impl DeterministicHost for DefaultDeterministicHost {
    fn logical_context(&self) -> LogicalContext {
        self.ctx.clone()
    }

    fn fill_rng(&mut self, buf: &mut [u8]) {
        self.rng.fill_bytes(buf);
    }
}

pub fn create_deterministic_engine() -> Engine {
    let mut config = Config::new();
    config.cranelift_nan_canonicalization(true);
    config.relaxed_simd_deterministic(true);
    config.static_memory_maximum_size(0);
    config.epoch_interruption(false);
    Engine::new(&config).expect("deterministic engine config must be valid")
}

pub fn execute_deterministic(
    engine: &Engine,
    host: &mut dyn DeterministicHost,
    workload: &WorkloadSpec,
) -> antml:Result<DeeOutput> {
    let module = Module::from_file(engine, &workload.module_path)?;
    let mut linker = Linker::new(engine);

    linker.func_wrap("env", "get_logical_time", {
        let ctx = host.logical_context();
        move || -> u64 { ctx.logical_time }
    })?;

    let mut store = Store::new(engine, ());
    let instance = linker.instantiate(&mut store, &module)?;
    let func = instance
        .get_typed_func::<i32, i32>(&mut store, &workload.entrypoint)?;

    let input_json = serde_json::to_string(&workload.input_json)?;
    let input_len = input_json.len() as i32;

    let alloc: TypedFunc<i32, i32> = instance
        .get_typed_func(&mut store, "alloc")
        .expect("alloc function required");
    let input_ptr = alloc.call(&mut store, input_len)?;

    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("memory export required");
    memory.write(&mut store, input_ptr as usize, input_json.as_bytes())?;

    let result_ptr = func.call(&mut store, input_ptr)?;

    let len_func: TypedFunc<(), i32> = instance
        .get_typed_func(&mut store, "get_result_len")
        .expect("get_result_len function required");
    let result_len = len_func.call(&mut store, ())?;

    let mut result_buf = vec![0u8; result_len as usize];
    memory.read(&mut store, result_ptr as usize, &mut result_buf)?;

    let result_json: serde_json::Value = serde_json::from_slice(&result_buf)?;

    let mut hasher = Sha384::new();
    hasher.update(&result_buf);
    let digest_sha384 = format!("{:x}", hasher.finalize());

    Ok(DeeOutput {
        stdout: String::new(),
        stderr: String::new(),
        result_json,
        digest_sha384,
    })
}
