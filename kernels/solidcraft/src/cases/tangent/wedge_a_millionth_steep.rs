use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((-5.0, -5.0, -10.0), (5.0, 5.0, 0.0))?;
    // the plane z = y·10⁻⁶ through the x axis: level along x, rising along (0, 1, 10⁻⁶); its
    // normal x × y points up, to the half-space kept
    let rise = v3(0.0, 1.0, 1e-6) * (1.0 / (1.0f64 + 1e-12).sqrt());
    let plane = Plane { origin: Vec3::ZERO, x: Vec3::X, y: rise };
    let (above, _) = solidcraft_kernel::half_space(&cube, &plane, true)?;
    above.ok_or_else(|| KernelError::Failed(format!("{EMPTY}nothing of the cube lies above the plane")))
}
