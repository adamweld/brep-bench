//! The box from the origin to (10, 8, 6) shelled outward to a wall 1 thick, open at its top, its
//! outside rounded: the box grown by 1 and cut off at z = 6, less the box.
//!
//! springrep's outward shell moves each face out and extends the moved faces until they meet, so
//! its edges are sharp. The rounded wall is every point outside the box within 1 of it, below the
//! plane of the top: the box's offset by 1 (its faces moved out, each convex edge rounded to a
//! quarter cylinder and each corner to an eighth of a ball), less the half-space above z = 6, less
//! the box.
use springrep::boolean::{difference, half_space};
use springrep::edit::offset;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(10., 8., 6.), "box").map_err(e)?;
    let grown = offset(&b, 1., "grown").map_err(e)?;
    // everything of the grown box above the plane of the open top
    let above = half_space(&grown, Vec3::new(0., 0., 6.), Vec3::Z).map_err(e)?;
    let cup = difference(&grown, &above).map_err(e)?;
    difference(&cup, &b).map_err(e)
}
