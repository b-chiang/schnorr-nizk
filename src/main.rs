extern crate rand_core;
extern crate sha2;

use curve25519_dalek::constants;
use curve25519_dalek::ristretto::{RistrettoBasepointTable, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::{Identity, IsIdentity};
use sha2::Sha512;

use rand_core::{CryptoRng, OsRng};

struct Transcript {
    u: RistrettoPoint,
    c: Scalar,
    z: Scalar,
}

fn bytes(g: &RistrettoPoint, h: &RistrettoPoint, u: &RistrettoPoint) -> Vec<u8> {
    [
        g.compress().to_bytes(),
        h.compress().to_bytes(),
        u.compress().to_bytes(),
    ]
    .concat()
}

fn prove(g: &RistrettoBasepointTable, x: &Scalar, h: &RistrettoPoint) -> Transcript {
    let mut csprng = OsRng;
    let r = Scalar::random(&mut csprng);
    let u = &r * g;
    let c = Scalar::hash_from_bytes::<Sha512>(&bytes(&g.basepoint(), h, &u));
    let z = r + c * x;
    Transcript { u, c, z }
}

fn verify(g: &RistrettoBasepointTable, h: &RistrettoPoint, t: &Transcript) -> bool {
    let c = Scalar::hash_from_bytes::<Sha512>(&bytes(&g.basepoint(), h, &t.u));
    c == t.c && g * &t.z == t.u + h * c
}

fn main() {
    assert!(true);
    let B = &constants::RISTRETTO_BASEPOINT_TABLE;
    let B_ = &constants::RISTRETTO_BASEPOINT_POINT;
    let l = &constants::BASEPOINT_ORDER;
    let A = l * B;
    let a = Scalar::from(2u64);
    let b = a * a;
    // println!("b = {:?}", b);
    let c = B * &a;
    assert!(A.is_identity());

    let d = B_ + B_;
    let e = &a * B;
    // println!("d = {:?}, e = {:?}", d, e);
    assert_eq!(d, e);
    assert_ne!(d + B_, e);

    let x = &Scalar::from(3u64);
    let h = x * B;
    let t1 = prove(B, x, &h);

    let h2 = x + x;
    let t2 = prove(B, &h2, &h);
    let h3 = l * B;
    assert_eq!(h3, RistrettoPoint::identity());
    let t3 = prove(B, l, &h3);
    let h4 = &h2 * B;
    let t4 = prove(B, x, &h4);
    println!("{}", verify(B, &h, &t1));
    println!("{}", verify(B, &h, &t2));
    println!("{}", verify(B, &h3, &t3));
    println!("{}", verify(B, &h, &t4));
    // assert_eq!(d + B_, e);
    // println!("P = {:?}", P);
}
