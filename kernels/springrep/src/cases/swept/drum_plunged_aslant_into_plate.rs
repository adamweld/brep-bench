//! The plate (0, −5, 0)–(20, 5, 4) less everything a drum passes through as it slides by 8 along
//! (0.3, 0, √0.91): the cylinder of radius 2 about the line y = 0, z = −5, from x = 6 to 14.
//!
//! springrep has no sweep of a solid. The slide has no part along y, so a line along it through the
//! drum keeps its y and crosses the drum's section there, a rectangle, from side to side: through
//! the middle plane z = −5, or through an end. So the drum sweeps itself where it starts and stops,
//! and its middle section (6 ≤ x ≤ 14, |y| ≤ 2) and its two end discs carried along the slide, three
//! oblique prisms (`shared::swept`, a slide).
use springrep::boolean::difference;
use springrep::math::Vec2;
use springrep::primitive::cylinder;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::swept::{boxed, rect, shifted, slid, unite, v3};
use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let plate = boxed([0.0, -5.0, 0.0], [20.0, 5.0, 4.0], "plate")?;
    let axis = frame([6.0, 0.0, -5.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?;
    let drum = cylinder(&axis, 2.0, 8.0, "drum").map_err(e)?;
    let v = v3(0.3, 0.0, 0.91f64.sqrt()) * 8.0;
    let middle = slid(&rect(6.0, -2.0, 14.0, 2.0), &at([0.0, 0.0, -5.0]), v, "pocket")?;
    let disc = Profile::circle(Vec2::ZERO, 4.0);
    let near = slid(&disc, &axis, v, "pocket")?;
    let far = slid(&disc, &axis.translated(v3(8.0, 0.0, 0.0)), v, "pocket")?;
    let pocket = unite(&[&drum, &middle, &near, &far, &shifted(&drum, v)])?;
    difference(&plate, &pocket).map_err(e)
}
