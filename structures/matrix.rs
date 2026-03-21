const N: usize = 100;

#[derive(Clone)]
struct Matrix {
    mat: [[u64; N]; N],
    size: usize,
}

impl Matrix {
    fn new(size: usize) -> Self {
        Matrix {
            mat: [[INF; N]; N],
            size,
        }
    }

    fn mul(&self, other: &Matrix) -> Matrix {
        let mut res = Matrix::new(self.size);
        for i in 0..self.size {
            for k in 0..self.size {
                if self.mat[i][k] == INF {
                    continue;
                }
                for j in 0..self.size {
                    if other.mat[k][j] == INF {
                        continue;
                    }
                    let sum = self.mat[i][k] + other.mat[k][j];
                    if sum < res.mat[i][j] {
                        res.mat[i][j] = sum;
                    }
                }
            }
        }
        res
    }

    fn pow(&self, mut k: usize) -> Matrix {
        // For (min, +) semiring
        // Identity matrix I has I_ii = 0 and I_ij = INF (i != j)
        // because A * I = A means min(A_ik + I_kj) = A_ij.
        // If I_jj = 0, then A_ij + 0 = A_ij.
        let mut res = Matrix::new(self.size);
        for i in 0..self.size {
            res.mat[i][i] = 0;
        }

        let mut base = self.clone();
        while k > 0 {
            if k % 2 == 1 {
                res = res.mul(&base);
            }
            base = base.mul(&base);
            k /= 2;
        }
        res
    }
}
