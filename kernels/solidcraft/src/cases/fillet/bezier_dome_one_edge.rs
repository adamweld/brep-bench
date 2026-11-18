use crate::cases::nurbs::bezier_dome::block;
use crate::solids::*;

/// The block under the dome of `nurbs/bezier-dome`, its edge in the side y = 0 rounded: picked
/// halfway along it, at u = ½, where the cubic of heights 4, 6, 5, 3 stands at 5.
pub fn build() -> Result<Body, crate::Refusal> {
    Ok(solidcraft_kernel::fillet(&block()?, &[v3(10.0, 0.0, 5.0)], 1.0)?)
}
