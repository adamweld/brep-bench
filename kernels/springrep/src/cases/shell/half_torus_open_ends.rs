//! The half with y >= 0 of the solid torus about the z axis, its tube of radius 2 about a middle
//! circle of radius 6, shelled inward to a wall 0.5 thick, open at both flat ends: a curved pipe.
use std::f64::consts::PI;

use springrep::edit::Wall;
use springrep::profile::{NamedSegment, Segment};
use springrep::topo::Brep;
use springrep::math::Vec2;

use crate::shared::shell::{shelled, turned};

pub fn build() -> Result<Brep, String> {
    // the tube's circle in the xz plane, turned half a turn from +x through +y to -x; a partial
    // turn's caps are `start` and `end`
    let tube = Segment::Circle {
        center: Vec2::new(6., 0.),
        radius: 2.,
        ccw: true,
    };
    let half = turned(vec![NamedSegment::new(tube, "tube")], PI, "half")?;
    shelled(&half, &["half.start", "half.end"], 0.5, Wall::Inward, "shell")
}
