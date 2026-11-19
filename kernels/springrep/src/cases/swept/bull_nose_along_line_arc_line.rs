//! The box (−10, −10, −10)–(40, 40, 0) less everything a bull-nose mill passes through as its
//! upright axis runs from (0, 0) along x to (20, 0), a quarter turn about the upright line through
//! (20, 10) to (30, 10), and along y to (30, 30): a cylinder of radius 2 from z = −1.5 to 5, its
//! bottom corner rounded to 0.5.
//!
//! springrep has no sweep of a solid. The mill is a solid of revolution about an upright axis and
//! the path is level: on a straight leg every line along it through the mill crosses the mill's
//! section square to the path through its axis, and on the bend every circle about the bend's axis
//! meets the mill in one arc across that section. So the mill sweeps that section carried along the
//! path (springrep's sweep along legs and bends), and itself where it starts and stops.
use std::f64::consts::{FRAC_PI_2, PI};

use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::sweep::{Leg, along};
use springrep::topo::Brep;

use crate::shared::swept::{arc, boxed, line, outline, shifted, unite, upright, v3};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let block = boxed([-10.0, -10.0, -10.0], [40.0, 40.0, 0.0], "block")?;
    let meridian = outline(vec![
        line((0.0, -1.5), (1.5, -1.5), "bottom"),
        arc((1.5, -1.0), 0.5, -FRAC_PI_2, FRAC_PI_2, "round"),
        line((2.0, -1.0), (2.0, 5.0), "side"),
        line((2.0, 5.0), (0.0, 5.0), "top"),
        line((0.0, 5.0), (0.0, -1.5), "axis"),
    ]);
    let mill = upright(&meridian, 0.0, 0.0, "mill")?;
    // the section through the axis square to +x: sketch x along y, sketch y up
    let section = outline(vec![
        line((-1.5, -1.5), (1.5, -1.5), "floor"),
        arc((1.5, -1.0), 0.5, -FRAC_PI_2, FRAC_PI_2, "round"),
        line((2.0, -1.0), (2.0, 5.0), "wall"),
        line((2.0, 5.0), (-2.0, 5.0), "top"),
        line((-2.0, 5.0), (-2.0, -1.0), "wall"),
        arc((-1.5, -1.0), 0.5, PI, FRAC_PI_2, "round"),
    ]);
    let start = frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?;
    let legs = [
        Leg::Line { length: 20.0 },
        Leg::Arc { radius: 10.0, angle: FRAC_PI_2, axis: Vec3::Z },
        Leg::Line { length: 20.0 },
    ];
    let channel = along(&section, &start, &legs, "groove").map_err(e)?;
    let groove = unite(&[&mill, &channel, &shifted(&mill, v3(30.0, 30.0, 0.0))])?;
    difference(&block, &groove).map_err(e)
}
