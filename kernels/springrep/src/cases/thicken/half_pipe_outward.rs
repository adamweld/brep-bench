//! The half y >= 0 of the cylinder of radius 5 about the z axis from z = 0 to z = 10, a sheet,
//! thickened by 1 away from the axis: the half tube between radii 5 and 6.
use std::f64::consts::PI;

use springrep::math::Vec2;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::thicken::thickened;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the sheet is the curved face of the half cylinder, the half disk y >= 0 of radius 5 extruded
    // 10 along z; its flat face, y = 0, holds the sheet's normals at its straight edges
    let half_disk = Profile::outline(vec![
        NamedSegment::arc(Vec2::ZERO, 5., 0., PI, "skin"),
        NamedSegment::line(Vec2::new(-5., 0.), Vec2::new(5., 0.), "flat"),
    ]);
    let half = extrude(&half_disk, &at([0., 0., 0.]), 10., "pipe").map_err(e)?;
    thickened(&half, &["pipe.skin"], 1.)
}
