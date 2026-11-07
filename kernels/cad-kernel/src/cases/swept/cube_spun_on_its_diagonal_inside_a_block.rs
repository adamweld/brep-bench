//! The block (−3, −3, −3)–(3, 3, 3) with a sealed void: everything the cube (−1, −1, −1)–(1, 1, 1)
//! passes through as it turns a whole turn about its own long diagonal, the line through
//! (−1, −1, −1) and (1, 1, 1).
use crate::sketch::block;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::unit;
use std::f64::consts::TAU;

const STOCK: V = [3., 3., 3.];
const CUBE: V = [1., 1., 1.];

pub fn build() -> Result<Brep, String> {
    let stock = block([-STOCK[0], -STOCK[1], -STOCK[2]], STOCK)?;
    let cube = block([-CUBE[0], -CUBE[1], -CUBE[2]], CUBE)?;
    // the turn about the diagonal: a screw that does not advance, a whole turn over a roll of τ
    let path = Trajectory::Twist(Screw { origin: [-CUBE[0], -CUBE[1], -CUBE[2]],
                                         axis: unit(CUBE), ratio: 1., advance: 0. });
    // the kernel's closed swept solid cut from the block on one arrangement: turned whole, the
    // cube's sweep is its solid of revolution, the edges meeting the axis turned into cones and
    // the six skew to it into a hyperboloid (a cube's flat faces mesh exactly, at any bar)
    let size = stock.size();
    cut_from(&stock, &cube, &path, [0., TAU], size, tolerance(size))
        .map_err(|why| format!("the cube's sweep not built: {why:?}"))
}
