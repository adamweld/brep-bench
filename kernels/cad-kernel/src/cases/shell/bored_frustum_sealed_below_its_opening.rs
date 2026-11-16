//! A cone frustum on the z axis, the disk of radius 6 at z = 0 up to the disk of radius 3 at z = 4,
//! bored through along the axis by a hole of radius 1, shelled inward to a wall 1 thick, open at
//! its top: the moved side and the moved bore close on one another at z = 11/3, below the top, so
//! the cavity is a closed ring and the top stays whole.
use crate::sketch::faces_through;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, prism};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

const BASE_RADIUS: f64 = 6.;
const TOP_RADIUS: f64 = 3.;
const HEIGHT: f64 = 4.;
const BORE_RADIUS: f64 = 1.;
const THICKNESS: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let frustum = cone([0., 0., 0.], [0., 0., 1.], BASE_RADIUS, TOP_RADIUS, HEIGHT)?;
    // the drill runs on past both ends, so its ends cut nothing
    let drill = prism(&disc([0., 0., -1.], [0., 0., 1.], BORE_RADIUS), 0., HEIGHT + 2.)?;
    let bored = combined(&frustum, &drill, Op::Cut, 0.)?;
    // the top, the annulus between the bore and the rim, picked by a point on it
    let open = faces_through(&bored, &[[0., (BORE_RADIUS + TOP_RADIUS) / 2., HEIGHT]])?;
    shelled(&bored, &open, THICKNESS, Side::In, Join::Round)
}
