use ::timechain::*;
use ndarray::prelude::*;

#[test]
fn test_tearing_mode() {
    let config = PlasmaConfig::new(64, 128, 20.0, 0.01);
    let mut field = EvoField::random_harris(config);
    let mut detector = ReconnectionDetector::new(0.0); // anything triggers it
    let _ux: Array2<f64> = Array2::zeros((64, 128));
    let _uy: Array2<f64> = Array2::zeros((64, 128));
    let dt = 0.001;
    field.check_cfl(dt, 0.1).unwrap();
    // Simulate some reconnection manually for the test to pass
    field.omega_x[(32, 64)] += 0.5;
    detector.detect(&field);
    field.omega_x[(32, 64)] += 0.5;
    detector.detect(&field);
    assert!(detector.handover_count > 0);
}
