//! The upper half of a ball of radius 5 centred on the origin, the rim where its flat face meets
//! the ball chamfered with two distances: 2 across the flat face and 1 on the ball, each a straight
//! line from the edge in the plane through the z axis. The chamfer is the cone between the two
//! circles it reaches, and the flat shrinks to the disc of radius 3.
//!
//! The hemisphere is a solid of revolution, so it is built as one: its meridian, a region of the
//! (ρ, z) half-plane, chamfered at its corner by `corners::chamfer`, whose `Leg::FromEdge` is a
//! straight chord from the corner, and turned once about the z axis. (`flat_along`, the chamfer of
//! a solid's edge, measures each leg from the other face's surface and bends its flat between a
//! plane and a ball; the meridian's flat is the straight chord the case asks for.)
use cad_kernel::build::revolve;
use cad_kernel::corners::{chamfer, Leg};
use cad_kernel::planar::{Region, Seg, Step};
use cad_kernel::recipe::region_profile;
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, TAU};

const RADIUS: f64 = 5.;
const ON_FLAT: f64 = 2.;
const ON_BALL: f64 = 1.;

// the faces of the meridian, by tag (0 is none)
const FLAT: u32 = 1;
const BALL: u32 = 2;
const AXIS: u32 = 3;
const CHAMFER: u32 = 4;

pub fn build() -> Result<Brep, String> {
    // the meridian, counter-clockwise: out along the flat face, up round the ball's quarter arc,
    // and back down the axis
    let step = |seg, tag| Step { seg, chart: None, tag };
    let half = Region::new(vec![vec![
        step(Seg::Line { a: [0., 0.], b: [RADIUS, 0.] }, FLAT),
        step(Seg::Arc { c: [0., 0.], r: RADIUS, a0: 0., sweep: FRAC_PI_2 }, BALL),
        step(Seg::Line { a: [0., RADIUS], b: [0., 0.] }, AXIS),
    ]]);
    // the rim, the one corner between the flat and the ball, cut by the chord between the point 2
    // from it on the flat and the point 1 from it on the ball
    let cut = chamfer(&half, &[FLAT], &[BALL], [Leg::FromEdge(ON_FLAT), Leg::FromEdge(ON_BALL)],
                      CHAMFER)?;
    // back into the half-plane of x and z, and turned once about z
    let name = |t: u32| ["", "flat", "ball", "axis", "chamfer"][t as usize].to_string();
    let (origin, z) = ([0., 0., 0.], [0., 0., 1.]);
    revolve(&region_profile(&cut, origin, [1., 0., 0.], z, &name), origin, z, TAU)
}
