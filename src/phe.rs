
//use std::ops::{Add, Sub, Mul, Div, Rem, Shl, Shr};
use std::ops::{Add, Sub, Mul, Div, Rem, Shr, Neg};
use std::marker::Sized;
use num_traits as num;

// pub trait Identities {
//     fn _zero() -> Self;
//     fn _one() -> Self;
//
//     fn is_zero(&Self) -> bool;
//     fn is_even(&Self) -> bool;
// }

pub trait NumberTests {
    fn is_zero(&Self) -> bool;
    fn is_even(&Self) -> bool;
    fn is_negative(me: &Self) -> bool;
}

pub trait ModularArithmetic
where
    Self: Clone + Sized,
    Self: num::Zero + num::One + NumberTests + Neg<Output=Self>,
    for<'a>    &'a Self: Mul<Self, Output=Self>,
    for<'a,'b> &'a Self: Mul<&'b Self, Output=Self>,
    for<'a,'b> &'a Self: Div<&'b Self, Output=Self>,
    for<'a>        Self: Rem<&'a Self, Output=Self>,
    for<'a,'b> &'a Self: Rem<&'b Self, Output=Self>,
    for<'a,'b> &'a Self: Add<&'b Self, Output=Self>,
                   Self: Sub<Self, Output=Self>,
    for<'b>        Self: Sub<&'b Self, Output=Self>,
    for<'a,'b> &'a Self: Sub<&'b Self, Output=Self>,
    Self: Shr<usize, Output=Self>,
{

    fn modinv(a: &Self, prime: &Self) -> Self {
        let r = a % prime;
        let ref d = if NumberTests::is_negative(&r) {
            let r = r.neg();
            -Self::egcd(prime, &r).2
        } else {
            Self::egcd(prime, &r).2
        };
        (prime + d) % prime
    }

    fn modpow(x: &Self, e: &Self, prime: &Self) -> Self {
        let mut mx = x.clone();
        let mut me = e.clone();
        let mut acc = Self::one();
        while !NumberTests::is_zero(&me) {
            if NumberTests::is_even(&me) {
                // even
                // no-op
            }
            else {
                // odd
                acc = (&acc * &mx) % prime;
            }
            mx = (&mx * &mx) % prime;  // waste one of these by having it here but code is simpler (tiny bit)
            me = me >> 1;
        }
        acc
    }

    fn egcd(a: &Self, b: &Self) -> (Self, Self, Self) {
        if NumberTests::is_zero(b) {
            (a.clone(), Self::one(), Self::zero())
        } else {
            let q = a / b;
            let r = a % b;
            let (d, s, t) = Self::egcd(b, &r);
            let new_t = s - &t * q;
            (d, t, new_t)
        }
    }
}


// pub trait Int where
//     Self: Add<Self, Output=Self>,
//
//     Self: Sub<Self, Output=Self>,
//     for<'a> Self: Sub<&'a Self, Output=Self>,
//
//     Self: Mul<Output=Self>,
//     for<'a> &'a Self: Mul<Self, Output=Self>,
//     for<'b> Self: Mul<&'b Self, Output=Self>,
//     // for<'a> &'a Self: Mul<Self, Output=Self>,
//     // for<'a, 'b> &'a Self: Mul<&'b Self, Output=Self>,
//
//     Self: Div<Output=Self>,
//
//     Self: Rem<Output=Self>,
//     for<'a> Self: Rem<&'a Self, Output=Self>,
//
//     Self: ModularArithmetic,
//     Self: Clone
// {}

pub trait Samplable {
    fn sample(upper: &Self) -> Self;
}

pub trait PartiallyHomomorphicScheme {
    type Plaintext;
    type Ciphertext;
    type EncryptionKey;
    type DecryptionKey;
    fn encrypt(&Self::EncryptionKey, &Self::Plaintext) -> Self::Ciphertext;
    fn decrypt(&Self::DecryptionKey, &Self::Ciphertext) -> Self::Plaintext;
    fn rerandomise(&Self::EncryptionKey, &Self::Ciphertext) -> Self::Ciphertext;
    fn add(&Self::EncryptionKey, &Self::Ciphertext, &Self::Ciphertext) -> Self::Ciphertext;
    fn mult(&Self::EncryptionKey, &Self::Ciphertext, &Self::Plaintext) -> Self::Ciphertext;
}

pub trait KeyGeneration {
    type EncryptionKey;
    type DecryptionKey;
    fn keypair(bit_length: usize) -> (Self::EncryptionKey, Self::DecryptionKey);
}
