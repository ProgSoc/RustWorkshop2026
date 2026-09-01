use std::{
    fmt,
    ops::{Add, Mul},
};

#[derive(Clone, Copy, Debug)]
struct SquareMatrix<const N: usize> {
    matrix: [[i64; N]; N],
}

impl<const N: usize> From<[[i64; N]; N]> for SquareMatrix<N> {
    fn from(item: [[i64; N]; N]) -> Self {
        Self { matrix: item }
    }
}

impl<const N: usize> Add for SquareMatrix<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            matrix: std::array::from_fn(|i| {
                std::array::from_fn(|j| self.matrix[i][j] + other.matrix[i][j])
            }),
        }
    }
}

impl<const N: usize> Mul for SquareMatrix<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            matrix: std::array::from_fn(|i| {
                std::array::from_fn(|j| {
                    (0..N)
                        .map(|k| self.matrix[i][k] * other.matrix[k][j])
                        .sum::<i64>()
                })
            }),
        }
    }
}

// Bonus if you want to implement the `Display` trait.
impl<const N: usize> fmt::Display for SquareMatrix<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for i in 0..N {
            let line: Vec<String> = self.matrix[i]
                .iter()
                .map(|element| element.to_string())
                .collect();
            if let Result::Err(err) = writeln!(f, "{}", line.join(" ")) {
                return Result::Err(err);
            }
        }
        Ok(())
    }
}

pub fn main() {
    let m1 = SquareMatrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    let m2 = SquareMatrix::from([[1, 0, 0], [0, -1, 0], [0, 0, -1]]);
    println!("{:?}", m1 + m2);
    println!("{:?}", m2 * m2);
    println!("{:?}", m1 + m2 * m2);
}
