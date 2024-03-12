//! > Using lambdaworks, compute the public key associated
//! with the secret key 0x6C616D6264617370 with the BLS12-381 elliptic curve.
use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::{
    BLS12381Curve, BLS12381FieldElement,
};
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::unsigned_integer::element::UnsignedInteger;

const SCALAR_THE: UnsignedInteger<2> =
    UnsignedInteger::<2>::from_hex_unchecked("0x6C616D6264617370");

fn main() {
    let point_the = BLS12381Curve::generator()
        .operate_with_self(SCALAR_THE)
        .to_affine();
    let [x, y, z] = point_the.coordinates();
    assert!(z == &BLS12381FieldElement::one());

    /* As it isn't specified should it be used SEC1 encoding/compression or other algorithm
    to represent the point as the public key, let's just print out its coordinates. */
    println!("The public key point coordinates are");
    println!("x:{}", x);
    println!("y:{}", y);
}
