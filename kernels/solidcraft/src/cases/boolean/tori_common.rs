use crate::solids::*;

pub fn build() -> R<Body> {
    let lower = solidcraft_kernel::torus(v3(0.0, 0.0, -1.0), 5.0, 2.0)?;
    let upper = solidcraft_kernel::torus(v3(0.0, 0.0, 1.0), 5.0, 2.0)?;
    common(&lower, &upper)
}
