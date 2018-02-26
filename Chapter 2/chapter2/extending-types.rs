trait Sawtooth {
    fn sawtooth(&self) -> Self;
}

impl Sawtooth for f64 {
    fn sawtooth(&self) -> f64 {
        self - self.floor()
    }
}

fn main() {
    println!("{}", 2.34f64.sawtooth());
}
