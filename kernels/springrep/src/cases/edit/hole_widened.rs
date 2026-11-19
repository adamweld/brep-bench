//! The plate from the origin to (40, 30, 8) drilled through by a hole of radius 5; the hole's wall
//! then offset 2 into the plate, to radius 7.
use std::collections::HashMap;

use springrep::boolean::difference;
use springrep::edit::offset_faces;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::edit::faces;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([0., 0., 0.]), Vec3::new(40., 30., 8.), "plate").map_err(e)?;
    let drill = cylinder(&at([20., 15., -1.]), 5., 10., "hole").map_err(e)?;
    let drilled = difference(&plate, &drill).map_err(e)?;
    // a face moves along its outward normal, which on a hole's wall points at the axis: into the
    // plate is negative
    let by: HashMap<_, _> = faces(&drilled, &["hole.side"])?.into_iter().map(|f| (f, -2.)).collect();
    offset_faces(&drilled, &by).map_err(e)
}
