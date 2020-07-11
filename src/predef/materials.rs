use crate::primitives::material::Material;
use crate::predef::colors;

#[allow(dead_code)]
pub const GLASS: Material = Material::Refractive {
    spec_col: colors::WHITE,
    refr_col: colors::WHITE,
    ior: 1.5,
};

#[allow(dead_code)]
pub const WATER: Material = Material::Refractive {
    spec_col: colors::WHITE,
    refr_col: colors::WHITE,
    ior: (4./3.),
};
