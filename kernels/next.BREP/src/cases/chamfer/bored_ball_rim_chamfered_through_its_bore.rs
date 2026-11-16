//! A ball of radius 5 about the origin bored through along the z axis by a hole of radius 2, the
//! upper circle where the bore meets the ball chamfered with two distances, 2√21 down the bore (its
//! whole length, so the bore is consumed) and 1 on the ball, each a straight line from the edge in
//! the plane square to it: the ball cut by the cone from the lower rim to the circle on the ball 1
//! in a straight line from the upper rim.
use crate::solids::boolean::cut;
use crate::solids::fillet::distances_ball_first;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{chamfer_edges_asymmetric, make_cylinder_brep, make_sphere_brep, BrepSolid};

const ON_BALL: f64 = 1.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    // The bore runs past both sides of the ball.
    let bore = make_cylinder_brep(p(0.0, 0.0, -10.0), p(0.0, 0.0, 1.0), 2.0, 20.0)?;
    let bored = cut(&ball, &bore)?;
    // The bore's whole length, from its upper rim to its lower.
    let down_bore = 2.0 * 21f64.sqrt();
    // The upper rim, by a point on it. The kernel's two-distance chamfer sets back its first
    // distance on the edge's first face, each distance a chord in the plane square to the edge.
    let rim = p(0.0, -2.0, 21f64.sqrt());
    let (d1, d2) = distances_ball_first(&bored, rim, p(0.0, 0.0, 0.0), ON_BALL, down_bore)?;
    Ok(chamfer_edges_asymmetric(&bored, &[rim], None, d1, d2, None)?)
}
