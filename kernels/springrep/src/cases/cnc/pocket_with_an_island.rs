//! A plate, the box (0, 0, −20)–(160, 110, 0), less what three upright cutters sweep: a pocket
//! (the rectangle (20, 20)–(140, 90) rounded to 12, round an island of radius 15 about (60, 55))
//! cleared 12 deep by a flat end mill of radius 5 in raster passes and laps at three depths, a
//! bore of radius 9 milled 6 deep in the island by a flat end mill of radius 3, and four holes
//! pecked by a drill of radius 4.
use springrep::boolean::difference_all;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::profile::region::Region;
use springrep::topo::Brep;

use crate::shared::cnc::{Cutter, flat_lap, flat_slide, plate, rounded};
use crate::shared::e;

/// The x where the line at `y` crosses the wall path, the rectangle (25, 25)–(135, 85) with its
/// corners rounded to 7: its left and right ends.
fn wall_ends(y: f64) -> (f64, f64) {
    let into_corner = (32. - y).max(y - 78.).max(0.);
    let back = 7. - (49. - into_corner * into_corner).sqrt();
    (25. + back, 135. - back)
}

pub fn build() -> Result<Brep, String> {
    let stock = plate([0., 0., -20.], [160., 110., 0.])?;
    let wall = rounded(&[[25., 25.], [135., 25.], [135., 85.], [25., 85.]], 7.)?;
    let island = Region::of(&Profile::circle(Vec2::new(60., 55.), 40.)).map_err(e)?;
    let mut cuts = Vec::new();
    // Every plunge and rise lies, inside the plate, within the sweep of the cut it starts or
    // ends (a pass's slot, a lap's ring), so only the cuts at depth are built.
    for z in [-4., -8., -12.] {
        for k in 0..15 {
            let y = 26. + 4. * k as f64;
            let (left, right) = wall_ends(y);
            let dy: f64 = y - 55.;
            let passes = if dy.abs() < 20. {
                let half = (400. - dy * dy).sqrt();
                vec![(left, 60. - half), (60. + half, right)]
            } else {
                vec![(left, right)]
            };
            for (a, b) in passes {
                cuts.push(flat_slide([a, y], [b, y], 5., 30., z)?);
            }
        }
        cuts.extend(flat_lap(&wall, 5., 30., z)?);
        cuts.extend(flat_lap(&island, 5., 30., z)?);
    }
    // the bore: down the axis, out to the circle of radius 6, round it and back, at two depths
    let bore = Cutter::Flat { radius: 3., length: 30. };
    let circle = Region::of(&Profile::circle(Vec2::new(60., 55.), 12.)).map_err(e)?;
    cuts.push(bore.plunge(60., 55., -6., 5.)?);
    for z in [-3., -6.] {
        cuts.push(flat_slide([60., 55.], [66., 55.], 3., 30., z)?);
        cuts.extend(flat_lap(&circle, 3., 30., z)?);
    }
    // each hole's pecks run up and down its axis: the drill from its lowest tip to its highest
    let drill = Cutter::Drill { radius: 4., length: 30. };
    for (x, y) in [(8., 8.), (152., 8.), (152., 102.), (8., 102.)] {
        cuts.push(drill.plunge(x, y, -14., 5.)?);
    }
    difference_all(&stock, &cuts.iter().collect::<Vec<_>>()).map_err(e)
}
