use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArkheError {
    #[error("Erro de algebra linear: {0}")]
    LinalgError(#[from] ndarray_linalg::error::LinalgError),
    #[error("Erro de serialização: {0}")]
    Serialization(String),
    #[error("Erro de rede IO: {0}")]
    Network(String),
    #[error("Violação de topologia: Helicidade inconsistente")]
    TopologyViolation,
    #[error("Condição CFL violada: dt={dt} > max={max}")]
    CflViolation { dt: f64, max: f64 },
    #[error("Sombra inativa ou vazia")]
    EmptyShadow,
    #[error("Bloco rejeitado por alta entropia de fase (Sybil/Desalinhamento)")]
    HighPhaseEntropy,
    #[error("Assinatura Pós-Quântica inválida ou corrompida")]
    QuantumSignatureInvalid,
    #[error("Falha no estabelecimento de chave quântica (KEM)")]
    QuantumKemFailure,
}
