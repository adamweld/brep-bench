//! A plate, the box from (−10, −10, 0) to (10, 10, 4), cut right through by a core drill: the tube
//! between 1.5 and 2.5 from the line through (0, 0, 2) along u = (sin 30°, 0, cos 30°), running
//! from 18 to 6 before (0, 0, 2), slid along u by 24.
//!
//! solidcraft-kernel has no sweep of a solid, so the tube's sweep is built by elementary geometry:
//! a prism slid along its own axis sweeps the same prism lengthened by the slide, here the tube
//! from 18 before (0, 0, 2) to 18 past it, its annulus extruded along u. The core it leaves is a
//! separate solid of the result.
use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((-10.0, -10.0, 0.0), (10.0, 10.0, 4.0))?;
    let a = 30f64.to_radians();
    let u = v3(a.sin(), 0.0, a.cos());
    let start = Plane::from_normal(v3(0.0, 0.0, 2.0) - u * 18.0, u)
        .ok_or_else(|| KernelError::Invalid("the tube's axis".into()))?;
    let annulus = Region2 {
        outer: Loop2::circle(Vec2::ZERO, 2.5),
        holes: vec![Loop2::circle(Vec2::ZERO, 1.5)],
    };
    cut(&plate, &extrude(&start, annulus, 0.0, 36.0)?)
}
