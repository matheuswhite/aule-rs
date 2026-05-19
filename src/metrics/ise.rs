use crate::{
    block::Block,
    math::{float_point::FloatPoint, number::Number},
    prelude::SimulationState,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ISE<T> {
    acc: T,
    n: usize,
}

impl<T> ISE<T>
where
    T: Number,
{
    pub fn value(&self) -> T {
        if self.n == 0 {
            T::zero()
        } else {
            self.acc
                .clone()
                .scale(<T::Alpha as FloatPoint>::recip_of_count(self.n))
        }
    }
}

impl<T> Block for ISE<T>
where
    T: Number,
{
    type Input = T;
    type Output = T;

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        self.acc += input * input;
        self.n += 1;
        input
    }

    fn reset(&mut self) {
        self.acc = T::zero();
        self.n = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Simulation;
    use num_complex::Complex;

    fn first_state() -> SimulationState {
        Simulation::new(1e-3, 1.0)
            .next()
            .expect("simulation should yield at least one state")
    }

    // ───────────────────────────── f32 ─────────────────────────────

    #[test]
    fn f32_empty_returns_zero() {
        let ise = ISE::<f32>::default();
        assert_eq!(ise.value(), 0.0_f32);
    }

    #[test]
    fn f32_averages_squared_values() {
        let mut ise = ISE::<f32>::default();
        let state = first_state();
        ise.block(2.0, state);
        ise.block(-3.0, state);
        ise.block(4.0, state);
        // (4 + 9 + 16) / 3 = 29 / 3
        let expected = 29.0_f32 / 3.0;
        assert!((ise.value() - expected).abs() < 1e-6);
    }

    #[test]
    fn f32_block_returns_input_unchanged() {
        let mut ise = ISE::<f32>::default();
        let state = first_state();
        assert_eq!(ise.block(-5.0, state), -5.0_f32);
        assert_eq!(ise.block(7.0, state), 7.0_f32);
    }

    #[test]
    fn f32_reset_clears_accumulator() {
        let mut ise = ISE::<f32>::default();
        let state = first_state();
        ise.block(10.0, state);
        ise.block(-6.0, state);
        ise.reset();
        assert_eq!(ise.value(), 0.0_f32);
        ise.block(2.0, state);
        assert_eq!(ise.value(), 4.0_f32);
    }

    // ───────────────────────────── f64 ─────────────────────────────

    #[test]
    fn f64_empty_returns_zero() {
        let ise = ISE::<f64>::default();
        assert_eq!(ise.value(), 0.0_f64);
    }

    #[test]
    fn f64_averages_squared_values() {
        let mut ise = ISE::<f64>::default();
        let state = first_state();
        ise.block(1.0, state);
        ise.block(-2.0, state);
        ise.block(3.0, state);
        // (1 + 4 + 9) / 3 = 14 / 3
        let expected = 14.0_f64 / 3.0;
        assert!((ise.value() - expected).abs() < 1e-12);
    }

    #[test]
    fn f64_reset_clears_accumulator() {
        let mut ise = ISE::<f64>::default();
        let state = first_state();
        ise.block(8.0, state);
        ise.block(-4.0, state);
        ise.reset();
        assert_eq!(ise.value(), 0.0_f64);
    }

    // ───────────────────────────── Complex<f32> (c32) ─────────────────────────────

    #[test]
    fn complex_f32_empty_returns_zero() {
        let ise = ISE::<Complex<f32>> {
            acc: Complex::new(0.0, 0.0),
            n: 0,
        };
        assert_eq!(ise.value(), Complex::new(0.0_f32, 0.0));
    }

    #[test]
    fn complex_f32_divides_accumulated_by_count() {
        let ise = ISE::<Complex<f32>> {
            acc: Complex::new(6.0, 8.0),
            n: 2,
        };
        assert_eq!(ise.value(), Complex::new(3.0_f32, 4.0));
    }

    // ───────────────────────────── Complex<f64> (c64) ─────────────────────────────

    #[test]
    fn complex_f64_empty_returns_zero() {
        let ise = ISE::<Complex<f64>> {
            acc: Complex::new(0.0, 0.0),
            n: 0,
        };
        assert_eq!(ise.value(), Complex::new(0.0_f64, 0.0));
    }

    #[test]
    fn complex_f64_divides_accumulated_by_count() {
        let ise = ISE::<Complex<f64>> {
            acc: Complex::new(9.0, -12.0),
            n: 3,
        };
        assert_eq!(ise.value(), Complex::new(3.0_f64, -4.0));
    }
}
