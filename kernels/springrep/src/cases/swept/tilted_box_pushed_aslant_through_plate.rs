//! The plate (−15, −15, 0)–(15, 15, 4) less everything a tilted box passes through as it slides
//! along (4, 2, 18): the box about (−2, −1, −8) reaching 2, 1.5 and 1 along e₁ = (2, 1, 2)/3,
//! e₂ = (−2, 2, 1)/3 and e₃ = (1, 2, −2)/3.
//!
//! springrep has no sweep of a solid. A point the box passes through lies in the box where it
//! starts, or behind one of the three faces it leads with, which it has carried along the slide;
//! so the sweep is the box and those faces carried along (4, 2, 18), oblique prisms
//! (`shared::swept::slid_cuboid`).
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, slid_cuboid, v3};

pub fn build() -> Result<Brep, String> {
    let plate = boxed([-15.0, -15.0, 0.0], [15.0, 15.0, 4.0], "plate")?;
    let axes = [
        v3(2.0, 1.0, 2.0) * (1.0 / 3.0),
        v3(-2.0, 2.0, 1.0) * (1.0 / 3.0),
        v3(1.0, 2.0, -2.0) * (1.0 / 3.0),
    ];
    let hole = slid_cuboid(v3(-2.0, -1.0, -8.0), axes, [2.0, 1.5, 1.0], v3(4.0, 2.0, 18.0), "box")?;
    difference(&plate, &hole).map_err(e)
}
