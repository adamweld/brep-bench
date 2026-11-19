//! The plate from the origin to (40, 30, 8) drilled through by two holes of radius 3, 10 apart; both
//! walls then offset 3 into the plate, to radius 6, so that the holes run into one opening.
use std::collections::HashMap;

use springrep::boolean::difference_all;
use springrep::edit::offset_faces;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::edit::faces;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([0., 0., 0.]), Vec3::new(40., 30., 8.), "plate").map_err(e)?;
    let left = cylinder(&at([15., 15., -1.]), 3., 10., "left").map_err(e)?;
    let right = cylinder(&at([25., 15., -1.]), 3., 10., "right").map_err(e)?;
    let drilled = difference_all(&plate, &[&left, &right]).map_err(e)?;
    // into the plate, away from each axis, is against the walls' outward normals
    let walls = faces(&drilled, &["left.side", "right.side"])?;
    let by: HashMap<_, _> = walls.into_iter().map(|f| (f, -3.)).collect();
    offset_faces(&drilled, &by)
        .map_err(|err| format!("springrep's offset_faces declines: {}", e(err)))
}
