//! The plate [0, 100]×[0, 20]×[0, 10] drilled nine times along z, holes of radius 2 at y = 10 and
//! x = 10, 20, … 90, each hole its own cut.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::boolean::between;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let mut plate = between([0., 0., 0.], [100., 20., 10.], "plate")?;
    for k in 1..=9 {
        let drill = cylinder(&at([10. * k as f64, 10., -1.]), 2., 12., "hole").map_err(e)?;
        plate = difference(&plate, &drill).map_err(e)?;
    }
    Ok(plate)
}
