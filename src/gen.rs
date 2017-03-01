/// Signal generators

use num_complex::{Complex, Complex32};
use signals::{Signal, signal};

/// Impulse signal
/// x[n] = 1 if n == 0
/// x[n] = 0 if n > 0
pub fn impulse(size: usize) -> Signal {
    let mut vs: Vec<Complex32> = Vec::with_capacity(size);
    vs.push(Complex::new(1., 0.));
    for _ in 1..size {
        vs.push(Complex::new(0., 0.));
    }
    signal(vs)
}


/// Step signal
/// x[n] = 1 if n >= 0
/// x[n] = 0 if n < 0
pub fn step(size: usize) -> Signal {
    let mut vs: Vec<Complex32> = Vec::with_capacity(size);
    for _ in 0..size {
        vs.push(Complex::new(1., 0.));
    }
    signal(vs)
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use num_complex::{Complex};
    use super::*;

    #[test]
    fn test_impulse() {
        let v = impulse(4);
        assert!(v.ndim() == 1);
        assert!(v.len() == 4);
        assert!(v == signal(vec![Complex::new(1., 0.),
                                 Complex::new(0., 0.),
                                 Complex::new(0., 0.),
                                 Complex::new(0., 0.)]));
    }
}