//! A pipe of radius 2 along three edges of the cube of side 30, (0, 0, 0) → (30, 0, 0) →
//! (30, 30, 0) → (30, 30, 30), mitered at both corners.
use springrep::boolean::union_all;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::sweep::behind;
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // each run is a cylinder carried past its corners and cut back to the miter planes, which
    // bisect the corners: x + y = 30 and y + z = 30
    let run = |from: [f64; 3], dir: [f64; 3], side: [f64; 3]| {
        cylinder(&frame(from, dir, side)?, 2., 40., "run").map_err(e)
    };
    let first = run([0., 0., 0.], [1., 0., 0.], [0., 0., 1.])?;
    let first = behind(&first, [30., 0., 0.], [1., 1., 0.])?;
    let second = run([30., -5., 0.], [0., 1., 0.], [0., 0., 1.])?;
    let second = behind(&second, [30., 0., 0.], [-1., -1., 0.])?;
    let second = behind(&second, [30., 30., 0.], [0., 1., 1.])?;
    let third = run([30., 30., -5.], [0., 0., 1.], [1., 0., 0.])?;
    let third = behind(&third, [30., 30., 0.], [0., -1., -1.])?;
    let third = behind(&third, [30., 30., 30.], [0., 0., 1.])?;
    union_all(&[&first, &second, &third]).map_err(e)
}
