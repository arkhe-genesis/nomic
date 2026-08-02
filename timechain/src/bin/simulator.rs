use ::timechain::consensus::ConsensusEngine;
use ::timechain::network::{P2PNode, PeerInfo};
use ::timechain::utxo::Utxo;
use ::timechain::*;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("=== Timechain (Arkhe) — Simulação com P2P, Finality e UTXO ===");

    // Configuração base
    let config = PlasmaConfig::new(64, 128, 20.0, 0.01);

    // In-memory UTXO ledger (hash -> UTXO)
    let _utxo_ledger: Arc<Mutex<HashMap<[u8; 32], Utxo>>> = Arc::new(Mutex::new(HashMap::new()));

    // Set up nodes
    let num_nodes = 3;
    let mut nodes = Vec::new();
    let base_port = 8000;

    for i in 0..num_nodes {
        let addr: SocketAddr = format!("127.0.0.1:{}", base_port + i).parse().unwrap();
        let node_result = P2PNode::new(addr, config).await;
        let node = Arc::new(Mutex::new(node_result.unwrap()));
        nodes.push(node);
    }

    // Connect nodes in a ring
    for i in 0..num_nodes {
        let node = &nodes[i];
        let next_node_idx = (i + 1) % num_nodes;
        let next_addr: SocketAddr = format!("127.0.0.1:{}", base_port + next_node_idx).parse().unwrap();
        let mut n = node.lock().await;
        n.peers.insert(next_addr, PeerInfo::new(next_node_idx as u64));
    }

    // Start node tasks
    for node_arc in nodes.iter() {
        let node_clone = node_arc.clone();
        tokio::spawn(async move {
            // Need to drop the lock so it doesn't block forever
            let mut n_owned;
            {
                let mut n = node_clone.lock().await;
                // Just create a brand new one to swap with, binding to port 0
                let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
                n_owned = P2PNode::new(addr, n.config).await.unwrap();
                // Swap it
                std::mem::swap(&mut *n, &mut n_owned);
            }
            let _ = n_owned.run().await;
        });
    }

    // Simulation logic
    let total_steps = 100;
    let engine = ConsensusEngine::new(0.5);

    for step in 0..total_steps {
        for _node in &nodes {
            // Simulated field changes inside the node itself in a real implementation.
            let _h = 0.0;
            let _finalized = engine.check_finality(step as u64, 0.0);
        }
        sleep(Duration::from_millis(5)).await;
    }

    println!("✅ Simulação concluída com múltiplos nós via P2P.");
}
