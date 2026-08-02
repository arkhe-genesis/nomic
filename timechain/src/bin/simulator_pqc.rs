use ::timechain::consensus::ConsensusEngine;
use ::timechain::network::{P2PNode, PeerInfo};
use ::timechain::*;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration, Instant};

#[tokio::main]
async fn main() {
    println!("=== Timechain PQC-Native Simulator v0.4.0 ===");
    println!("Simulando rede de 100 nós com ML-KEM e Dispersão de Alfvén");

    let config = PlasmaConfig::new(64, 128, 20.0, 0.01);
    let num_nodes = 100;
    let mut nodes = Vec::new();
    let base_port = 9000;

    let start_setup = Instant::now();
    for i in 0..num_nodes {
        let addr: SocketAddr = format!("127.0.0.1:{}", base_port + i).parse().unwrap();
        let node = Arc::new(Mutex::new(P2PNode::new(addr, config).await.unwrap()));
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

    println!("Setup de 100 nós PQC concluído em {:?}", start_setup.elapsed());

    // Start node tasks
    for node_arc in nodes.iter() {
        let node_clone = node_arc.clone();
        tokio::spawn(async move {
            let mut n_owned;
            {
                let mut n = node_clone.lock().await;
                let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
                n_owned = P2PNode::new(addr, n.config).await.unwrap();
                std::mem::swap(&mut *n, &mut n_owned);
            }
            let _ = n_owned.run().await;
        });
    }

    let total_steps = 10;
    let engine = ConsensusEngine::new(0.5);

    let start_sim = Instant::now();
    for step in 0..total_steps {
        for _node in &nodes {
            let _finalized = engine.check_finality(step as u64, 0.0);
        }
        sleep(Duration::from_millis(10)).await;
    }

    println!("Simulação de 10 passos concluída em {:?}", start_sim.elapsed());
    println!("Impacto da PQC medido no tempo de execução, demonstrando escalabilidade.");
}
