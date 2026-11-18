use crate::solids::*;

/// The cylinder of radius 5 on the z axis from z = −1 to 10, kept above the plane z = 0 and
/// below the plane z = x tan 30°, which meets it along the y axis. That plane rises along
/// (cos 30°, 0, sin 30°) and is level along y, so its normal x × y = (−sin 30°, 0, cos 30°)
/// points up, away from the kept part.
pub fn build() -> Result<Body, crate::Refusal> {
    let cylinder = zcyl(0.0, 0.0, 5.0, -1.0, 10.0)?;
    let (above, _) = solidcraft_kernel::half_space(&cylinder, &Plane::XY, true)?;
    let above = above.ok_or("nothing of the cylinder lies above z = 0")?;
    let (s, c) = 30f64.to_radians().sin_cos();
    let slant = Plane { origin: Vec3::ZERO, x: v3(c, 0.0, s), y: Vec3::Y };
    let (hoof, _) = solidcraft_kernel::half_space(&above, &slant, false)?;
    Ok(hoof.ok_or("nothing of the cylinder lies below the slanting plane")?)
}
