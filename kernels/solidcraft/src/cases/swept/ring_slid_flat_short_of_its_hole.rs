//! The solid a ring fills as it slides without turning along x by 6: the torus about the z axis
//! centred on the origin, the circle through its tube's centres of radius 6, its tube of radius 2.
//!
//! solidcraft-kernel has no sweep of a solid, so the ring's sweep is built by elementary geometry,
//! level by level. At height z the ring is an annulus, radii 6 ∓ √(4 − z²). A point the annulus's
//! outer disc slid over is missed by the annulus only if its whole path back lies in the hole:
//! being a disc, if it lies in the hole where the ring starts and where it stops. So the sweep is
//! what the ring with its hole filled sweeps (as in `ring-slid-flat-past-its-hole`) less the lens
//! the hole shares at its two places. The slide, 6, is less than the hole's width, 8 at the
//! least, so the lens runs all the way through. The hole is the revolve of its half section, here
//! carried on to z = ±3 as the cylinder of radius 6, past the ring's flat top and bottom, so that
//! the lens is cut through them.
use super::ring_slid_flat_past_its_hole::filled_slid;
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// The ring's hole about the vertical line through (`x`, 0), carried on to z = ±3.
fn hole(x: f64) -> R<Body> {
    // the half section in (r, z): in to the tube's inner half
    let segs = vec![
        Seg2::Line { a: v2(0.0, -3.0), b: v2(6.0, -3.0) },
        Seg2::Line { a: v2(6.0, -3.0), b: v2(6.0, -2.0) },
        Seg2::Arc { center: v2(6.0, 0.0), radius: 2.0, start: -FRAC_PI_2, sweep: -PI },
        Seg2::Line { a: v2(6.0, 2.0), b: v2(6.0, 3.0) },
        Seg2::Line { a: v2(6.0, 3.0), b: v2(0.0, 3.0) },
        Seg2::Line { a: v2(0.0, 3.0), b: v2(0.0, -3.0) },
    ];
    let plane = Plane { origin: v3(x, 0.0, 0.0), x: Vec3::X, y: Vec3::Z };
    let section = Region2 { outer: Loop2 { segs }, holes: vec![] };
    one(solidcraft_kernel::revolve(&plane, &[section], Vec2::ZERO, Vec2::Y, TAU)?)
}

pub fn build() -> R<Body> {
    let lens = common(&hole(0.0)?, &hole(6.0)?)?;
    cut(&filled_slid(6.0)?, &lens)
}
