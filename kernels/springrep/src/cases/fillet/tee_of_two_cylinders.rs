//! A tee of two solid cylinders, a bar of radius 6 along the x axis and a stem of radius 3 up the
//! z axis, the saddle-shaped junction where the stem leaves the bar rounded to radius 1 by a
//! rolling ball.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let along_x = frame([-20., 0., 0.], [1., 0., 0.], [0., 1., 0.])?;
    let bar = cylinder(&along_x, 6., 40., "bar").map_err(e)?;
    let stem = cylinder(&at([0., 0., 0.]), 3., 15., "stem").map_err(e)?;
    let tee = union(&bar, &stem).map_err(e)?;
    fillet(&tee, &[("bar.side", "stem.side")], 1., "round").map_err(e)
}
