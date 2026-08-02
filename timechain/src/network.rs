use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use std::time::Instant;

use crate::{EchoSignal, TimeBlock, PlasmaConfig, EvoField, ObserverState};
use crate::timechain::ChernSimonsOracle;
use crate::consensus::ConsensusEngine;
use crate::ArkheError;

use pqcrypto_kyber::kyber1024::{PublicKey as KemPubKey, SecretKey as KemSecKey, SharedSecret};
use pqcrypto_kyber::kyber1024;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use sha3::{Sha3_256, Digest};
use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _, Ciphertext as _, SharedSecret as _};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    Heartbeat { node_id: u64, field_hash: [u8; 32], phase_time: f64 },
    Echo { echo: EchoSignal, block: TimeBlock },
    BlockRequest { height: u64 },
    BlockResponse { block: TimeBlock },
    KemOffer { pub_key_bytes: Vec<u8> },
    KemAccept { ciphertext_bytes: Vec<u8> },
    EncryptedPayload { ciphertext: Vec<u8>, nonce: [u8; 12] },
}

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub node_id: u64,
    pub last_seen: Instant,
    pub packet_history: Vec<Duration>,
    pub measured_attachment: f64,
}

impl PeerInfo {
    pub fn new(node_id: u64) -> Self {
        Self {
            node_id,
            last_seen: Instant::now(),
            packet_history: Vec::with_capacity(100),
            measured_attachment: 1.0, // Desconhecido = desconfiança máxima
        }
    }
}

pub struct SecurePeerSession {
    pub shared_secret: SharedSecret,
    pub established_at: Instant,
}

pub struct P2PNode {
    pub socket: std::sync::Arc<UdpSocket>,
    pub node_id: u64,
    pub peers: HashMap<SocketAddr, PeerInfo>,
    pub config: PlasmaConfig,
    pub field: EvoField,
    pub observer: ObserverState,
    pub oracle: ChernSimonsOracle,
    pub consensus: ConsensusEngine,
    pub addr: SocketAddr,
    pub received_echos: HashMap<u64, Vec<EchoSignal>>,
    pub kem_secret_key: KemSecKey,
    pub kem_public_key: KemPubKey,
    pub secure_sessions: HashMap<SocketAddr, SecurePeerSession>,
}

impl P2PNode {
    pub async fn new(addr: SocketAddr, config: PlasmaConfig) -> Result<Self, ArkheError> {
        let socket = std::sync::Arc::new(UdpSocket::bind(addr).await
            .map_err(|e| ArkheError::Network(e.to_string()))?);
        let node_id = rand::random::<u64>();
        let field = EvoField::harris_sheet(config);
        let observer = ObserverState::new();
        let oracle = ChernSimonsOracle::new(0.001);
        let consensus = ConsensusEngine::new(0.5);
        let (kem_public_key, kem_secret_key) = kyber1024::keypair();
        Ok(Self {
            socket,
            node_id,
            peers: HashMap::new(),
            config,
            field,
            observer,
            oracle,
            consensus,
            addr,
            received_echos: HashMap::new(),
            kem_public_key,
            kem_secret_key,
            secure_sessions: HashMap::new(),
        })
    }

    /// Deriva a chave AES-256 a partir do segredo compartilhado via HKDF/SHA-3
    fn derive_aes_key(shared_secret: &SharedSecret) -> Key<Aes256Gcm> {
        let mut hasher = Sha3_256::new();
        hasher.update(shared_secret.as_bytes());
        let result = hasher.finalize();
        let key_bytes: [u8; 32] = result.into();
        *Key::<Aes256Gcm>::from_slice(&key_bytes)
    }

    /// Broadcast de eco PQC Criptografado com AEAD
    pub async fn broadcast_echo(&self, echo: EchoSignal, block: TimeBlock) -> Result<(), ArkheError> {
        let k_dominant = 1.0 + 10.0 * echo.strength.clamp(0.0, 1.0);
        let v0 = 1.0;
        let v_eco = v0 * k_dominant.powf(-1.0 / 3.0);
        let delay = 1.0 / (v_eco + 1e-6);

        let cleartext_packet = NetworkMessage::Echo { echo, block };
        let cleartext = bincode::serialize(&cleartext_packet)
            .map_err(|e| ArkheError::Serialization(e.to_string()))?;

        for (peer_addr, _) in self.peers.iter() {
            let socket = self.socket.clone();
            let target = *peer_addr;

            // Verifica se tem sessão segura com o peer
            if let Some(session) = self.secure_sessions.get(peer_addr) {
                let aes_key = Self::derive_aes_key(&session.shared_secret);
                let cipher = Aes256Gcm::new(&aes_key);
                let nonce_bytes: [u8; 12] = rand::random();
                let nonce = Nonce::from_slice(&nonce_bytes);

                if let Ok(ciphertext) = cipher.encrypt(nonce, cleartext.as_ref()) {
                    let packet = NetworkMessage::EncryptedPayload { ciphertext, nonce: nonce_bytes };
                    let data = bincode::serialize(&packet).unwrap();

                    tokio::spawn(async move {
                        sleep(Duration::from_secs_f64(delay * 0.01)).await;
                        let _ = socket.send_to(&data, target).await;
                    });
                }
            } else {
                // Envia oferta de KEM
                let offer = NetworkMessage::KemOffer { pub_key_bytes: self.kem_public_key.as_bytes().to_vec() };
                let data = bincode::serialize(&offer).unwrap();
                let _ = socket.send_to(&data, target).await;
            }
        }

        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), ArkheError> {
        use bincode::Options;
        let mut buf = [0u8; 65536];
        loop {
            let (len, src) = self.socket.recv_from(&mut buf).await
                .map_err(|e| ArkheError::Network(e.to_string()))?;

            let options = bincode::DefaultOptions::new().with_limit(65536);
            let packet: NetworkMessage = match options.deserialize(&buf[..len]) {
                Ok(p) => p,
                Err(_) => continue,
            };
            self.handle_message(packet, src).await?;
        }
    }

    async fn handle_message(&mut self, msg: NetworkMessage, src: SocketAddr) -> Result<(), ArkheError> {
        match msg {
            NetworkMessage::KemOffer { pub_key_bytes } => {
                if let Ok(pk) = KemPubKey::from_bytes(&pub_key_bytes) {
                    let (shared_secret, ciphertext) = kyber1024::encapsulate(&pk);
                    self.secure_sessions.insert(src, SecurePeerSession {
                        shared_secret,
                        established_at: Instant::now(),
                    });
                    let accept = NetworkMessage::KemAccept { ciphertext_bytes: ciphertext.as_bytes().to_vec() };
                    let data = bincode::serialize(&accept).unwrap();
                    let _ = self.socket.send_to(&data, src).await;
                }
            }
            NetworkMessage::KemAccept { ciphertext_bytes } => {
                if let Ok(ct) = kyber1024::Ciphertext::from_bytes(&ciphertext_bytes) {
                    let shared_secret = kyber1024::decapsulate(&ct, &self.kem_secret_key);
                    self.secure_sessions.insert(src, SecurePeerSession {
                        shared_secret,
                        established_at: Instant::now(),
                    });
                }
            }
            NetworkMessage::EncryptedPayload { ciphertext, nonce } => {
                if let Some(session) = self.secure_sessions.get(&src) {
                    let aes_key = Self::derive_aes_key(&session.shared_secret);
                    let cipher = Aes256Gcm::new(&aes_key);
                    let nonce_obj = Nonce::from_slice(&nonce);
                    if let Ok(cleartext) = cipher.decrypt(nonce_obj, ciphertext.as_ref()) {
                        use bincode::Options;
                        let options = bincode::DefaultOptions::new().with_limit(65536);
                        if let Ok(inner_msg) = options.deserialize::<NetworkMessage>(&cleartext) {
                            // Dispatch without boxed recursion
                            self.handle_decrypted_message(inner_msg, src).await?;
                        }
                    }
                }
            }
            _ => {
                // For non-encrypted messages, we handle them directly via decrypted logic
                self.handle_decrypted_message(msg, src).await?;
            }
        }
        Ok(())
    }

    async fn handle_decrypted_message(&mut self, msg: NetworkMessage, src: SocketAddr) -> Result<(), ArkheError> {
        match msg {
            NetworkMessage::Echo { echo, block } => {
                // PoPD: mede apego do peer pelo jitter de chegada
                if let Some(peer) = self.peers.get_mut(&src) {
                    peer.packet_history.push(Duration::from_secs_f64(echo.timestamp));
                    if peer.packet_history.len() >= 10 {
                        peer.measured_attachment = measure_attachment_from_jitter(&peer.packet_history);
                        peer.packet_history.clear();
                    }
                    // Rejeita ecos de peers com alto apego medido
                    if peer.measured_attachment > 0.7 {
                        return Ok(());
                    }
                }

                if self.oracle.verify(&block, &self.field) {
                    self.received_echos
                        .entry(block.height)
                        .or_default()
                        .push(echo.clone());
                    self.consensus.add_echo(echo.clone());
                    let _ = self.broadcast_echo(echo, block.clone()).await;
                    if self.check_finality(block.height) {
                        println!("[✅ FINALIDADE] Bloco {} finalizado", block.height);
                    }
                }
            }
            NetworkMessage::Heartbeat { node_id, field_hash: _, phase_time: _ } => {
                self.peers.entry(src).or_insert_with(|| PeerInfo::new(node_id));
            }
            _ => {}
        }
        Ok(())
    }

    fn check_finality(&self, height: u64) -> bool {
        let echos = self.received_echos.get(&height);
        if echos.is_none() || echos.unwrap().len() < 3 {
            return false;
        }
        let echos = echos.unwrap();
        let local_phase = self.field.helicity() % (2.0 * std::f64::consts::PI);
        let mut sum = num_complex::Complex64::new(0.0, 0.0);
        for echo in echos {
            let amp = echo.strength;
            let phase = echo.predicted_helicity % (2.0 * std::f64::consts::PI);
            let diff = (phase - local_phase).rem_euclid(2.0 * std::f64::consts::PI);
            sum += num_complex::Complex64::new(amp * diff.cos(), amp * diff.sin());
        }
        sum.norm() > 0.5 * echos.len() as f64
    }
}

/// PoPD: Calcula apego REAL de um peer baseado na entropia de Shannon dos intervalos
pub fn measure_attachment_from_jitter(packet_arrivals: &[Duration]) -> f64 {
    if packet_arrivals.len() < 10 {
        return 1.0; // Desconhecido = Desconfiança
    }

    let mut intervals = Vec::with_capacity(packet_arrivals.len() - 1);
    for i in 1..packet_arrivals.len() {
        intervals.push(
            (packet_arrivals[i].as_secs_f64() - packet_arrivals[i - 1].as_secs_f64()).abs(),
        );
    }

    let entropy = calculate_shannon_entropy(&intervals);
    let max_possible_entropy = (intervals.len() as f64).log2().max(1e-12);
    let normalized_entropy = (entropy / max_possible_entropy).clamp(0.0, 1.0);

    normalized_entropy
}

fn calculate_shannon_entropy(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let sum: f64 = values.iter().sum();
    if sum == 0.0 {
        return 0.0;
    }
    let mut entropy = 0.0;
    for &v in values {
        let p = v / sum;
        if p > 0.0 {
            entropy -= p * p.log2();
        }
    }
    entropy
}
