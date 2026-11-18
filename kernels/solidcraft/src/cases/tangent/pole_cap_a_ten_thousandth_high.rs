use crate::solids::*;

pub fn build() -> R<Body> {
    // the kernel's ball has its poles on the z axis: the loop runs round the pole at (0, 0, 5)
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    // the block above the plane z = 4.9999, past the ball on every side but its floor
    common(&ball, &block((-6.0, -6.0, 4.9999), (6.0, 6.0, 6.0))?)
}
