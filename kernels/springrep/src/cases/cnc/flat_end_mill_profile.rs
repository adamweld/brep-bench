//! A plate, the box (0, 0, −15)–(120, 90, 0), less what a flat end mill of radius 3 sweeps
//! running round a cross with its corners rounded to 6, at depths 3, 6 and 9, plunging from
//! (60, 10, 5) and rising back there.
use springrep::boolean::difference_all;
use springrep::topo::Brep;

use crate::shared::cnc::{Cutter, flat_lap, plate, rounded};
use crate::shared::e;

const CROSS: [[f64; 2]; 12] = [
    [45., 10.], [75., 10.], [75., 30.], [100., 30.], [100., 60.], [75., 60.],
    [75., 80.], [45., 80.], [45., 60.], [20., 60.], [20., 30.], [45., 30.],
];

pub fn build() -> Result<Brep, String> {
    let stock = plate([0., 0., -15.], [120., 90., 0.])?;
    let mill = Cutter::Flat { radius: 3., length: 20. };
    let path = rounded(&CROSS, 6.)?;
    let mut cuts = Vec::new();
    // the plunge to the first lap, each step down between laps, and the rise at the end
    cuts.push(mill.plunge(60., 10., -3., 5.)?);
    cuts.push(mill.plunge(60., 10., -6., -3.)?);
    cuts.push(mill.plunge(60., 10., -9., -6.)?);
    cuts.push(mill.plunge(60., 10., -9., 5.)?);
    for z in [-3., -6., -9.] {
        cuts.extend(flat_lap(&path, 3., 20., z)?);
    }
    difference_all(&stock, &cuts.iter().collect::<Vec<_>>()).map_err(e)
}
