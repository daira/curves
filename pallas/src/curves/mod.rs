use crate::{fp::Fp, fq::Fq};
use ark_ec::{
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
};
use ark_ff::{biginteger::BigInteger256, field_new, Zero};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct PallasParameters;

impl ModelParameters for PallasParameters {
    type BaseField = Fp;
    type ScalarField = Fq;
}

pub type Affine = GroupAffine<PallasParameters>;
pub type Projective = GroupProjective<PallasParameters>;

impl SWModelParameters for PallasParameters {
    /// COEFF_A = 0
    const COEFF_A: Fp = field_new!(Fp, BigInteger256([0x0, 0x0, 0x0, 0x0]));

    /// COEFF_B = 5
    const COEFF_B: Fp = field_new!(
        Fp,
        BigInteger256([
            0xa1a55e68ffffffed,
            0x74c2a54b4f4982f3,
            0xfffffffffffffffd,
            0x3fffffffffffffff
        ])
    );

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = 1
    const COFACTOR_INV: Fq = field_new!(
        Fq,
        BigInteger256([
            0x5b2b3e9cfffffffd,
            0x992c350be3420567,
            0xffffffffffffffff,
            0x3fffffffffffffff
        ])
    );

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G_GENERATOR_X, G_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

/// G_GENERATOR_X = -1
/// Encoded in Montgomery form, so the value here is -R mod p.
pub const G_GENERATOR_X: Fp = field_new!(
    Fp,
    BigInteger256([
        0x64b4c3b400000004,
        0x891a63f02533e46e,
        0x0000000000000000,
        0x0000000000000000
    ])
);

/// G1_GENERATOR_Y = 2
/// Encoded in Montgomery form, so the value here is 2R mod p.
pub const G_GENERATOR_Y: Fp = field_new!(
    Fp,
    BigInteger256([
        0xcfc3a984fffffff9,
        0x1011d11bbee5303e,
        0xffffffffffffffff,
        0x3fffffffffffffff
    ])
);
