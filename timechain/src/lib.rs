pub mod accelerate;
pub mod consensus;
pub mod error;
pub mod mhd;
pub mod network;
pub mod observer;
pub mod retro;
pub mod shadow;
pub mod storage;
pub mod timechain;
pub mod utxo;
pub mod svd_compress;

pub use error::ArkheError;
pub use mhd::{EvoField, PlasmaConfig, ReconnectionDetector};
pub use observer::ObserverState;
pub use retro::{EchoSignal, RetroCausalChannel};
pub use shadow::{Shadow, ShadowHealer};
pub use storage::{ShadowSnapshot, ShadowStore};
pub use timechain::{ChernSimonsOracle, TimeBlock};
pub use svd_compress::{CompressedSignature, CompressionParams, SvdCompressor};

pub const CHERN_SIMONS_KAPPA: f64 = 1.0;
pub const DEFAULT_VALIDATION_TOLERANCE: f64 = 1e-3;
