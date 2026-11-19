//! Refused. #3DBenchy is a reference shape, not a description: a pass is the original solid of
//! its designer's CAD file, all 1385 of its faces, within 2e-5. The case gives no construction
//! of it, only the reference, whose faces may not be taken in as the solid's own; what any layer
//! of SolidCraft can build from numbers read off it (a hull through its net, boxes, cylinders) is
//! a likeness, which the case grades wrong.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("the case's solid is #3DBenchy's original CAD model, 1385 faces, and the case gives no \
         construction of it: what any layer can build from numbers read off the reference is a \
         likeness, not that solid"
        .into())
}
