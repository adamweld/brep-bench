//! The cube [0, 30]³ drilled right through by fifteen holes of radius 2.5 on a grid: three along x
//! and three along y at z = 15, and nine along z, crossing three at a time.
use springrep::boolean::difference_all;
use springrep::topo::Brep;

use crate::shared::boolean::{between, rod};
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let cube = between([0., 0., 0.], [30., 30., 30.], "cube")?;
    let grid = [7.5, 15., 22.5];
    let mut drills = Vec::new();
    // each drill runs 1 past the faces at both ends
    for &a in &grid {
        drills.push(rod([-1., a, 15.], [1., 0., 0.], 2.5, 32., "along_x")?);
        drills.push(rod([a, -1., 15.], [0., 1., 0.], 2.5, 32., "along_y")?);
        for &b in &grid {
            drills.push(rod([a, b, -1.], [0., 0., 1.], 2.5, 32., "along_z")?);
        }
    }
    let drills: Vec<&Brep> = drills.iter().collect();
    difference_all(&cube, &drills).map_err(e)
}
