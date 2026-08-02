use ::timechain::*;
use ndarray::prelude::*;
use rayon::prelude::*;

#[test]
fn test_100_nodes() {
    let config = PlasmaConfig::new(8, 8, 10.0, 0.01); // smaller grid for fast test
    let fields: Vec<_> = (0..100).map(|_| EvoField::random_harris(config)).collect();
    let handovers: Vec<u32> = fields
        .par_iter()
        .map(|f| {
            let mut f = f.clone();
            let mut detector = ReconnectionDetector::new(0.0001);
            let _ux: Array2<f64> = Array2::zeros((8, 8));
            let _uy: Array2<f64> = Array2::zeros((8, 8));
            f.omega_x[(4, 4)] += 0.5;
            detector.detect(&f);
            f.omega_x[(4, 4)] += 0.5;
            detector.detect(&f);
            detector.handover_count
        })
        .collect();
    let total: u32 = handovers.iter().sum();
    println!("Total de handovers entre 100 nós: {}", total);
    assert!(total > 0);
}
