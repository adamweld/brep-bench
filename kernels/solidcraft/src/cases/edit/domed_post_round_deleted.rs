//! A post on the z axis, the cylinder of radius 4 from z = 0 to z = 6 capped by the part above
//! z = 6 of the ball of radius 5 about (0, 0, 3), its rim where the two meet rounded to radius 1;
//! then the round deleted, the cylinder and the dome run on until they meet in the rim again.
use crate::bench::{common, fuse};
use crate::shared::edit::{delete_rounds, round_the_rim, rounded};
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let dome = common(make_sphere([0.0, 0.0, 3.0], 5.0)?, make_box([-6.0, -6.0, 6.0], [12.0, 12.0, 3.0])?)?;
    let post = fuse(make_cylinder([0.0; 3], 4.0, 6.0)?, dome)?;
    // the rim's edges and then its round, by points all round it
    let rim = round_the_rim(4.0, 6.0);
    let post = rounded(&post, &rim, 1.0)?;
    delete_rounds(&post, &rim)
}
