
use rand;
use ramp;

use phe::*;

// use num_traits as num;
// #[derive(Debug,Clone)]
// pub struct RampBigInteger(pub ramp::Int);

impl Samplable for ramp::Int {
    fn sample(upper: &Self) -> Self {
        use ramp::RandomInt;
        let mut rng = rand::OsRng::new().unwrap();
        rng.gen_uint_below(upper)
    }
}

impl NumberTests for ramp::Int {
    fn is_zero(me: &Self) -> bool { me == &0_usize }
    fn is_even(me: &Self) -> bool { me.is_even() }
    fn is_negative(me: &Self) -> bool { me < &0_usize }
}

impl ModularArithmetic for ramp::Int {}

use super::abstractimpl::AbstractPlainPaillier;
pub type RampPlainPaillier = AbstractPlainPaillier<ramp::Int>;
