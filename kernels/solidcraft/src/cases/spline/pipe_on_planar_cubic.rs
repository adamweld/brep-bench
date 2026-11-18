use crate::solids::*;
use solidcore_record::Seg2;
use solidcraft_kernel::PathSeg;

/// The cubic as a path: the kernel's sweep follows a sketch's free-form curve exactly
/// (`PathSeg::Curve`), as SolidCraft's own Sweep hands it a sketch's Bézier.
pub fn bezier_path(plane: &Plane, curve: Seg2) -> Vec<PathSeg> {
    vec![PathSeg::Curve { plane: *plane, seg: curve }]
}

/// The disc of radius 20 in the plane square to the curve where it leaves the origin, along
/// (3, −4, 0), swept along the cubic.
pub fn build() -> R<Body> {
    let curve = Seg2::Cubic {
        p0: v2(0.0, 0.0),
        p1: v2(12.0, -16.0),
        p2: v2(32.0, -16.0),
        p3: v2(44.0, 0.0),
    };
    let start = Plane { origin: Vec3::ZERO, x: v3(0.8, 0.6, 0.0), y: Vec3::Z };
    solidcraft_kernel::sweep(&start, &disc((0.0, 0.0), 20.0), &bezier_path(&Plane::XY, curve))
}
