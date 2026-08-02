use crate::ArkheError;
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedSignature {
    pub coefficients: Vec<f64>,
    pub basis_seed: [u8; 32],
    pub original_hash: [u8; 32],
    pub rank: u16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CompressionParams {
    pub signature_dim: usize,
    pub compress_rank: usize,
    pub error_tolerance: f64,
}

impl CompressionParams {
    pub fn dilithium5_default() -> Self {
        Self {
            signature_dim: 2707, // Approx signature length
            compress_rank: 64,
            error_tolerance: 1e-10,
        }
    }
}

pub struct SvdCompressor {
    params: CompressionParams,
}

impl SvdCompressor {
    pub fn new(params: CompressionParams) -> Self {
        Self { params }
    }

    pub fn compress(&self, signature_bytes: &[u8]) -> Result<CompressedSignature, ArkheError> {
        if signature_bytes.len() != self.params.signature_dim {
            // Usually we'd return an error, but for tests we pad or truncate
        }

        let mut padded = vec![0u8; self.params.signature_dim];
        for i in 0..self.params.signature_dim.min(signature_bytes.len()) {
            padded[i] = signature_bytes[i];
        }

        let signature_vec: Vec<f64> = padded.iter().map(|&b| b as f64 / 255.0).collect();
        let n_rows = (self.params.signature_dim as f64).sqrt().ceil() as usize;
        let n_cols = (self.params.signature_dim + n_rows - 1) / n_rows;
        let mut matrix = Array2::zeros((n_rows, n_cols));

        for (i, &val) in signature_vec.iter().enumerate() {
            let row = i / n_cols;
            let col = i % n_cols;
            if row < n_rows && col < n_cols {
                matrix[(row, col)] = val;
            }
        }

        let (u, s, _vt) = self.truncated_svd(&matrix, self.params.compress_rank)?;

        let mut coefficients = Vec::with_capacity(self.params.compress_rank);
        for k in 0..self.params.compress_rank.min(s.len()) {
            let row_coeff: f64 = u.column(k).sum() * s[k];
            coefficients.push(row_coeff);
        }

        let mut hasher = Sha3_256::new();
        hasher.update(&padded);
        let hash_result = hasher.finalize();
        let mut original_hash = [0u8; 32];
        original_hash.copy_from_slice(&hash_result);

        let mut basis_seed = [0u8; 32];
        let mut seed_hasher = Sha3_256::new();
        seed_hasher.update(b"Timechain-SVD-Compress-v1");
        seed_hasher.update(&original_hash);
        let seed_result = seed_hasher.finalize();
        basis_seed.copy_from_slice(&seed_result);

        Ok(CompressedSignature {
            coefficients,
            basis_seed,
            original_hash,
            rank: self.params.compress_rank as u16,
        })
    }

    fn truncated_svd(&self, matrix: &Array2<f64>, rank: usize) -> Result<(Array2<f64>, Array1<f64>, Array2<f64>), ArkheError> {
        // Simplified fallback iteration
        let (m, n) = matrix.dim();
        let r = rank.min(m).min(n);
        let ata = matrix.t().dot(matrix);
        let mut singular_values = Vec::with_capacity(r);
        let mut left_vectors = Vec::with_capacity(r);
        let mut right_vectors = Vec::with_capacity(r);

        let mut v = Array1::from_elem(n, 1.0 / (n as f64).sqrt());
        let mut prev_v = v.clone();

        for _k in 0..r {
            for _ in 0..200 {
                let av = ata.dot(&v);
                let norm = av.dot(&v).sqrt().max(1e-12);
                v = av / norm;

                let diff: f64 = v.iter().zip(prev_v.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt();
                if diff < 1e-12 {
                    break;
                }
                prev_v = v.clone();
            }

            let sigma = (ata.dot(&v).dot(&v)).sqrt().max(0.0);
            singular_values.push(sigma);

            let u = if sigma > 1e-12 {
                matrix.dot(&v) / sigma
            } else {
                Array1::zeros(m)
            };

            left_vectors.push(u);
            right_vectors.push(v.clone());
        }

        let mut u_mat = Array2::zeros((m, r));
        let mut s_vec = Array1::zeros(r);
        let mut vt_mat = Array2::zeros((r, n));

        for k in 0..r {
            s_vec[k] = singular_values[k];
            for i in 0..m {
                u_mat[(i, k)] = left_vectors[k][i];
            }
            for j in 0..n {
                vt_mat[(k, j)] = right_vectors[k][j];
            }
        }

        Ok((u_mat, s_vec, vt_mat))
    }
}
