use crate::solids::*;

/// A ball of radius 5 centred on the origin, bored right through along the z axis by a hole of
/// radius 3, its rims the circles of radius 3 at z = ±4. The kernel's ball has its poles on its
/// z axis, so it is turned about the y axis until they lie along (3.1, 0, 4): each a little
/// outside a rim, the upper rim passing between the upper pole and the z axis. The drill runs
/// past both ends of the ball.
pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    // turning +z about +y by atan(3.1/4) takes it to (3.1, 0, 4)/|(3.1, 0, 4)|
    let ball = solidcraft_kernel::transform(&ball, Vec3::ZERO, Vec3::ZERO, Vec3::Y, 3.1f64.atan2(4.0))?;
    cut(&ball, &zcyl(0.0, 0.0, 3.0, -6.0, 6.0)?)
}
