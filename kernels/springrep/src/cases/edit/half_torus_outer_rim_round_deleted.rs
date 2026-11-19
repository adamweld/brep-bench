//! The half above z = 0 of the torus whose tube of radius 2 runs round the circle of radius 6, the
//! outer rim of its flat rounded to radius 1/2; the round then deleted, giving the half torus back.
use springrep::blend::fillet_edges;
use springrep::boolean::difference;
use springrep::math::{Point3, Vec3};
use springrep::measure::edge_at;
use springrep::primitive::{block, torus};
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ring = torus(&at([0., 0., 0.]), 6., 2., "ring").map_err(e)?;
    let below = block(&at([-10., -10., -3.]), Vec3::new(20., 20., 3.), "cut").map_err(e)?;
    let half = difference(&ring, &below).map_err(e)?;
    // the outer rim, picked where it passes through (8, 0, 0)
    let (rim, _) = edge_at(&half, Point3::new(8., 0., 0.)).ok_or("no edge at the outer rim")?;
    let rounded = fillet_edges(&half, &[rim], 0.5, "round").map_err(e)?;
    delete_feature(&rounded, "round")
}
