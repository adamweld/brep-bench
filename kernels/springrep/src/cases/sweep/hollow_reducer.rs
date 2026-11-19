//! A hollow square tube tapering along y from 24 × 24 (hole 20 × 20) at y = 0 to 12 × 12 (hole
//! 8 × 8) at y = 40, every face flat.
use springrep::boolean::difference;
use springrep::loft::{Section, loft};
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::{e, frame};

/// The ruled loft between squares of side `a` at y = 0 and `b` at y = 40, centred on the y axis.
fn tapered(a: f64, b: f64, feature: &str) -> Result<Brep, String> {
    let at = |y: f64, side: f64| -> Result<Section, String> {
        Ok(Section {
            profile: Profile::rect(Vec2::ZERO, side, side),
            frame: frame([0., y, 0.], [0., 1., 0.], [1., 0., 0.])?,
        })
    };
    loft(&[at(0., a)?, at(40., b)?], true, feature).map_err(e)
}

pub fn build() -> Result<Brep, String> {
    // a loft takes only its sections' outer loops: the outside and the hole are lofted apart
    let outside = tapered(24., 12., "outside")?;
    let hole = tapered(20., 8., "hole")?;
    difference(&outside, &hole).map_err(e)
}
