//! A plate, the box (0, 0, −16)–(120, 120, 0), less what a bull-nose mill (radius 4, corner 1.5)
//! sweeps running round a six-lobed flower of arcs about (60, 60) at depths 4, 8 and 10.

use springrep::boolean::difference_all;
use springrep::math::{Vec2, Vec3};
use springrep::sweep::{Leg, along};
use springrep::topo::Brep;

use crate::shared::cnc::{Cutter, plate};
use crate::shared::{e, frame};

/// The point `r` from (60, 60) at `deg` degrees.
fn polar(r: f64, deg: f64) -> Vec2 {
    let a = deg.to_radians();
    Vec2::new(60. + r * a.cos(), 60. + r * a.sin())
}

/// The angle between the directions from `c` to `a` and to `b`.
fn spread(c: Vec2, a: Vec2, b: Vec2) -> f64 {
    let (u, v) = (a - c, b - c);
    (u.dot(v) / (u.norm() * v.norm())).acos()
}

pub fn build() -> Result<Brep, String> {
    let stock = plate([0., 0., -16.], [120., 120., 0.])?;
    let mill = Cutter::BullNose { radius: 4., corner: 1.5, length: 25. };
    // a lobe of radius 12 about each point 30 out at 0°, 60°, …; a valley about each point 40 out
    // at 30°, 90°, …, tangent to the lobes either side, so of radius d − 12
    let (lobe_r, lobe) = (12., polar(30., 0.));
    let (valley_prev, valley) = (polar(40., -30.), polar(40., 30.));
    let valley_r = lobe.dist(valley) - lobe_r;
    // each arc's turn: the lobe's between its two valleys, the valley's between its two lobes
    let lobe_turn = spread(lobe, valley_prev, valley);
    let valley_turn = spread(valley, lobe, polar(30., 60.));
    // anticlockwise from the lobe at 0°'s outermost point: each lobe a left turn (about +z),
    // each valley a right one, the first lobe in two halves
    let left = |angle| Leg::Arc { radius: lobe_r, angle, axis: Vec3::Z };
    let right = Leg::Arc { radius: valley_r, angle: valley_turn, axis: -Vec3::Z };
    let mut legs = vec![left(lobe_turn / 2.)];
    for k in 0..6 {
        legs.push(right);
        legs.push(left(if k < 5 { lobe_turn } else { lobe_turn / 2. }));
    }
    let section = mill.silhouette()?;
    let mut cuts = vec![
        mill.plunge(102., 60., -4., 5.)?,
        mill.plunge(102., 60., -8., -4.)?,
        mill.plunge(102., 60., -10., -8.)?,
        mill.plunge(102., 60., -10., 5.)?,
    ];
    for z in [-4., -8., -10.] {
        // the section square to the path, heading +y: sketch x across it, sketch y up
        let start = frame([102., 60., z], [0., 1., 0.], [-1., 0., 0.])?;
        cuts.push(along(&section, &start, &legs, "lap").map_err(e)?);
    }
    difference_all(&stock, &cuts.iter().collect::<Vec<_>>()).map_err(e)
}
