//! The plate (0, −5, 0)–(20, 5, 4) less everything a drum passes through as it slides straight up
//! by 16: the cylinder of radius 2 about the line y = 0, z = −5, from x = 6 to 14.
//!
//! springrep has no sweep of a solid. Every upright line through the drum crosses it through its
//! middle plane z = −5, so the drum sweeps itself where it starts and stops and that section, the
//! rectangle 6 ≤ x ≤ 14, |y| ≤ 2, carried up 16: a box (`shared::swept`, a slide).
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::swept::{boxed, shifted, unite, v3};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let plate = boxed([0.0, -5.0, 0.0], [20.0, 5.0, 4.0], "plate")?;
    let drum = cylinder(&frame([6.0, 0.0, -5.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?, 2.0, 8.0, "drum")
        .map_err(e)?;
    let middle = boxed([6.0, -2.0, -5.0], [14.0, 2.0, 11.0], "slot")?;
    let slot = unite(&[&drum, &middle, &shifted(&drum, v3(0.0, 0.0, 16.0))])?;
    difference(&plate, &slot).map_err(e)
}
