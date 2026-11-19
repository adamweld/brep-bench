//! SolidCraft's Shell (`shell`) hollows inward only; solidcore-ops's [`shell`] beneath it takes a
//! positive thickness for a wall outward. The upper half ball of radius 5 shelled outward to a
//! wall 0.5 thick, open at its dome: the flat face moved 0.5 down, run on to the sphere.
use crate::bench::{cut, face_through, order};
use crate::Refusal;
use solidcore_ops::{make_box, make_sphere, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    let below = make_box([-6.0, -6.0, -6.0], [12.0, 12.0, 6.0])?;
    let half = cut(ball, below)?;
    // the dome, picked at a point on it off the pole and the rim
    let dome = face_through(&half, [0.0, 3.0, 4.0])?;
    shell(&half, &order(&half)?, &[dome], 0.5)?;
    Ok(half)
}
