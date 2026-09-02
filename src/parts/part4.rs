// TODO: Make this trivially copyable.
#[derive(Debug)]
struct SquareMatrix<const N: usize> {
    matrix: [[i64; N]; N],
}

// TODO: Implement the traits `From`, `Add`, and `Mul`.
/*
impl<const N: usize> From<[[i64; N]; N]> for SquareMatrix<N> {
    fn from(item: [[i64; N]; N]) -> Self {
        todo!()
    }
}

impl <const N: usize> Add for SquareMatrix<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {

    }
}
*/

pub fn main() {
    let i: SquareMatrix<2> = SquareMatrix {
        matrix: [[1, 0], [0, 1]],
    };
    println!(
        "[{} {}]\n[{} {}]",
        i.matrix[0][0], i.matrix[0][1], i.matrix[1][0], i.matrix[1][1]
    );

    // TODO: Uncomment the below driver code (and remove the above) when you're ready.
    /*
    let m1: SquareMatrix<3> = SquareMatrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    let m2: SquareMatrix<3> = SquareMatrix::from([[1, 0, 0], [0, -1, 0], [0, 0, -1]]);
    println!("{:?}", m1 + m2);
    println!("{:?}", m2 * m2);
    println!("{:?}", m1 + m2 * m2);
    */
}
