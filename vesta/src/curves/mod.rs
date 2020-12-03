use crate::{fp::Fp, fq::Fq};
use ark_ec::{
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
};
use ark_ff::{biginteger::BigInteger256, field_new, Zero};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct VestaParameters;

impl ModelParameters for VestaParameters {
    type BaseField = Fp;
    type ScalarField = Fq;
}

pub type Affine = GroupAffine<VestaParameters>;
pub type Projective = GroupProjective<VestaParameters>;

impl SWModelParameters for VestaParameters {
    /// COEFF_A = 0
    const COEFF_A: Fp = field_new!(Fp, BigInteger256([0x0, 0x0, 0x0, 0x0]));

    /// COEFF_B = 5
    const COEFF_B: Fp = field_new!(
        Fp,
        BigInteger256([
            0x96bc8c8cffffffed,
            0x74c2a54b49f7778e,
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
            0x34786d38fffffffd,
            0x992c350be41914ad,
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
        0x311bac8400000004,
        0x891a63f02652a376,
        0x0000000000000000,
        0x0000000000000000
    ])
);

/// G1_GENERATOR_Y = 2
/// Encoded in Montgomery form, so the value here is 2R mod p.
pub const G_GENERATOR_Y: Fp = field_new!(
    Fp,
    BigInteger256([
        0x2a0f9218fffffff9,
        0x1011d11bbcef61f1,
        0xffffffffffffffff,
        0x3fffffffffffffff
    ])
);
