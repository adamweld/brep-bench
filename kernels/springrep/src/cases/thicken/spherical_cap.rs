//! The cap of the sphere of radius 10 about the origin where z >= 6, a sheet, thickened by 2 away
//! from the center: the solid between the spheres of radius 10 and 12 inside the cone about z with
//! its apex at the origin through the cap's rim.
use springrep::topo::Brep;

use crate::shared::thicken::{arc, line, thickened, turned_whole};

pub fn build() -> Result<Brep, String> {
    // the sheet is the cap of a spherical sector, whose cone from the origin to the cap's rim, the
    // circle of radius 8 at z = 6, runs along the sphere's normal there
    let up = 6f64.atan2(8.);
    let sector = turned_whole(
        vec![
            line([0., 0.], [8., 6.], "cone"),
            arc([0., 0.], 10., up, std::f64::consts::FRAC_PI_2 - up, "cap"),
            line([0., 10.], [0., 0.], "axis"),
        ],
        "sector",
    )?;
    thickened(&sector, &["sector.cap"], 2.)
}
