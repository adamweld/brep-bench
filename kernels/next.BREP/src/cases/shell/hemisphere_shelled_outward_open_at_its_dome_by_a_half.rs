//! A half ball shelled outward by a half, open at its dome: the upper half of the ball of radius 5 centred on the origin: the part with z ≥ 0, shelled outward to a
//! wall 0.5 thick, open at its dome: the slice of the ball between z = −0.5 and z = 0. The kernel shells outward
//! for a negative thickness.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let above = make_box_brep(p(-6.0, -6.0, 0.0), 12.0, 12.0, 6.0)?;
    let half = common(&ball, &above)?;
    let dome = p(3.0, 0.0, 4.0);
    shell(&half, &[dome], -0.5)
}
