use super::paraboloid_tip_on_face::paraboloid;
use crate::solids::*;

pub fn build() -> R<Body> {
    // the kernel's ball has its poles on the z axis: its pole is the point of contact
    union(&paraboloid(10.0, 5.0)?, &solidcraft_kernel::sphere(v3(0.0, 0.0, 5.0), 5.0)?)
}
