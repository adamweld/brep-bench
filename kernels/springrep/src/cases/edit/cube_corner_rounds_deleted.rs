//! A cube of side 10, the three edges at its corner (10, 10, 10) rounded to radius 2 with a ball's
//! patch where they meet; all four faces then deleted, giving the cube back.
use springrep::blend::fillet;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let cube = block(&at([0., 0., 0.]), Vec3::new(10., 10., 10.), "cube").map_err(e)?;
    let edges = [
        ("cube.end", "cube.u_max"),
        ("cube.end", "cube.v_max"),
        ("cube.u_max", "cube.v_max"),
    ];
    let rounded = fillet(&cube, &edges, 2., "round").map_err(e)?;
    delete_feature(&rounded, "round")
}
