//! A post on the z axis, the cylinder of radius 4 from z = 0 to z = 6 capped by the part above
//! z = 6 of the ball of radius 5 about (0, 0, 3), the dome turned by 1.5 radians about the z axis
//! before the two are joined, so that the rim where they meet is two edges; the rim rounded to
//! radius 1, two faces, then both deleted, the cylinder and the dome run on until they meet in
//! the rim again.
use crate::bench::{common, fuse, rotated};
use crate::shared::edit::{delete_rounds, round_the_rim, rounded};
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let dome = common(make_sphere([0.0, 0.0, 3.0], 5.0)?, make_box([-6.0, -6.0, 6.0], [12.0, 12.0, 3.0])?)?;
    let dome = rotated(dome, [0.0; 3], [0.0, 0.0, 1.0], 1.5f64.to_degrees())?;
    let post = fuse(make_cylinder([0.0; 3], 4.0, 6.0)?, dome)?;
    // the rim's edges and then its rounds, by points all round it
    let rim = round_the_rim(4.0, 6.0);
    let post = rounded(&post, &rim, 1.0)?;
    delete_rounds(&post, &rim)
}
