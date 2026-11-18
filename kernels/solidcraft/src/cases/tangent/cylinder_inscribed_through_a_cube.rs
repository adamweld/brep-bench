use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((-5.0, -5.0, -5.0), (5.0, 5.0, 5.0))?;
    // the rod's circle on the plane x = −10, extruded 20 along +x, its seam started on the line
    // y = z of the wall, at 45° to the four lines of contact (the kernel's `cylinder` puts its
    // seam at an angle of its own)
    let plane = Plane::from_normal(v3(-10.0, 0.0, 0.0), Vec3::X)
        .ok_or_else(|| KernelError::Invalid("the rod's plane".into()))?;
    let seam = v3(0.0, 1.0, 1.0);
    let start = seam.dot(plane.y).atan2(seam.dot(plane.x));
    let circle = Region2 { outer: Loop2::circle_from(Vec2::ZERO, 5.0, start), holes: vec![] };
    let rod = extrude(&plane, circle, 0.0, 20.0)?;
    union(&cube, &rod)
}
