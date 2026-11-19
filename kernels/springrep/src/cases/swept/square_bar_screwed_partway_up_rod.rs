//! The rod of radius 5 about the z axis from z = 0 to z = 20, less everything the bar
//! 4 ≤ x ≤ 12, |y| ≤ 8, |z| ≤ 1 passes through as it is screwed about the z axis, rising 6 a turn,
//! from a turn of −120° to one of 600°, where it stops inside the rod.
//!
//! springrep has no sweep of a solid. The bar is 2 thick along the screw's axis, which the screw
//! rises in a turn of 120°: at each height it passes wholly, staying there through 120°, and what
//! it covers there is its level section turned through 120°, turned on with the height — a twisted
//! extrusion (`shared::swept::turned_bar_section`), up to where the bar's top stops. At the heights
//! the stopped bar spans, it covers that less what lies ahead of its tip where it stops: the
//! quarter ahead of the tip's leading half, cut away there. Only the bar within 5.5 of the axis
//! reaches the rod.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{
    arc, deg, level, line, outline, rod, turned_bar_section, twisted,
};

pub fn build() -> Result<Brep, String> {
    let (lead, t0, t1) = (6.0, deg(-120.0), deg(600.0));
    let rise = |t: f64| lead * t / (2.0 * PI);
    let delta = 2.0 * PI * 2.0 / lead;
    let section = turned_bar_section(4.0, None, 5.5, delta);
    let swept = twisted(&section, 1.0 + rise(t0), t0, lead, t1 - t0, "groove")?;
    // ahead of the stopped tip's leading half: x < 4, y > 0 in the bar's frame where it stops
    let h = 20f64.sqrt();
    let ahead = outline(vec![
        line((-6.0, 0.0), (4.0, 0.0), "stop"),
        line((4.0, 0.0), (4.0, h), "tip"),
        arc((0.0, 0.0), 6.0, h.atan2(4.0), PI - h.atan2(4.0), "reach"),
    ]);
    let ahead = extrude(&ahead, &level(-1.0 + rise(t1), t1), 3.0, "stop").map_err(e)?;
    let groove = difference(&swept, &ahead).map_err(e)?;
    difference(&rod()?, &groove).map_err(e)
}
