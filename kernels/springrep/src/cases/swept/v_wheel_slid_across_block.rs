//! The block (0, −5, 0)–(20, 5, 10) less everything a V wheel passes through as it slides along
//! the x axis by 36: the solid of revolution about the line through (−8, 0, 14) along y of the
//! triangle (0, 8), (−3 tan 30°, 11), (3 tan 30°, 11) in (y, z) on the plane x = −8.
//!
//! springrep has no sweep of a solid. A line along x through the wheel's bore meets the wheel on
//! both sides of it, and the slide, longer than the bore is wide, carries the part behind across
//! it: the wheel sweeps what the wheel with its bore filled does. That solid is convex and every
//! line along x through it crosses its section by the plane x = −8 through its axis, a hexagon; so
//! the sweep is it where it starts and stops and that hexagon carried along 36, a prism
//! (`shared::swept`, a slide).
use std::f64::consts::TAU;

use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::swept::{boxed, polygon, shifted, slid, unite, v3};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let block = boxed([0.0, -5.0, 0.0], [20.0, 5.0, 10.0], "block")?;
    let t = 3.0 * 30f64.to_radians().tan();
    // sketch (y, z) on the plane x = −8, which holds the wheel's axis z = 14
    let plane = frame([-8.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?;
    let filled = polygon(&[(0.0, 8.0), (t, 11.0), (t, 14.0), (-t, 14.0), (-t, 11.0)], "wheel");
    let wheel = revolve(&filled, &plane, v3(-8.0, 0.0, 14.0), Vec3::Y, TAU, "wheel").map_err(e)?;
    let section = polygon(
        &[(0.0, 8.0), (t, 11.0), (t, 17.0), (0.0, 20.0), (-t, 17.0), (-t, 11.0)],
        "groove",
    );
    let v = v3(36.0, 0.0, 0.0);
    let middle = slid(&section, &plane, v, "groove")?;
    let groove = unite(&[&wheel, &middle, &shifted(&wheel, v)])?;
    difference(&block, &groove).map_err(e)
}
