#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(m: &Matrix) -> Matrix {
  Matrix(
    m.0, m.2,
    m.1, m.3
  )
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    let matrix2 = transpose(&matrix);
    println!("{}", matrix);
    println!("{}", matrix2);
}
