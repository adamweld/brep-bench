//! A block, the box from the origin to (20, 10, 10), with a slot cut into its side y = 0 top to
//! bottom, from x = 8 to x = 12, running in to y = 5 and closed there by a half cylinder of
//! radius 2 about the line through (10, 5); then the slot's three faces deleted, the side
//! running on across its mouth and the top and bottom closing over it.
use crate::shared::edit::faces_nearest;
use crate::solids::*;

pub fn build() -> R<Body> {
    let sharp = block((0.0, 0.0, 0.0), (20.0, 10.0, 10.0))?;
    let straight = block((8.0, -1.0, -1.0), (12.0, 5.0, 11.0))?;
    let slotted = cut(&cut(&sharp, &straight)?, &zcyl(10.0, 5.0, 2.0, -1.0, 11.0)?)?;
    // the flats x = 8 and x = 12 by a point on each, and the half cylinder by points round it:
    // the kernel's cylinder starts its circle at 0.6131 radians from +x, so it is two faces
    let mut points = vec![v3(8.0, 2.0, 5.0), v3(12.0, 2.0, 5.0)];
    points.extend([0.3f64, 1.6, 2.9].iter().map(|a| v3(10.0 + 2.0 * a.cos(), 5.0 + 2.0 * a.sin(), 5.0)));
    let slot = faces_nearest(&slotted, &points)?;
    solidcraft_kernel::delete_faces(&slotted, &slot)
}
