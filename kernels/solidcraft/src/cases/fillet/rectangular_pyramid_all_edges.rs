use crate::solids::*;
use solidcraft_kernel::HalfSpace;

pub fn build() -> R<Body> {
    // the base, z ≥ 0; the long sides through y = ±6 and the apex, (0, ±2, 1)/√5 · p ≤ 12/√5;
    // the short ends through x = ±10 and the apex, (±6, 0, 5)/√61 · p ≤ 60/√61
    let (l, s) = (5f64.sqrt(), 61f64.sqrt());
    let mut faces = vec![HalfSpace { n: v3(0.0, 0.0, -1.0), d: 0.0 }];
    for k in [-1.0, 1.0] {
        faces.push(HalfSpace { n: v3(0.0, 2.0 * k / l, 1.0 / l), d: 12.0 / l });
        faces.push(HalfSpace { n: v3(6.0 * k / s, 0.0, 5.0 / s), d: 60.0 / s });
    }
    let pyramid = solidcraft_kernel::convex_polyhedron(&faces)?;
    // the midpoints of the base's four edges and of the four hips
    let mut edges = vec![v3(0.0, -6.0, 0.0), v3(0.0, 6.0, 0.0), v3(-10.0, 0.0, 0.0), v3(10.0, 0.0, 0.0)];
    for x in [-5.0, 5.0] {
        for y in [-3.0, 3.0] {
            edges.push(v3(x, y, 6.0));
        }
    }
    solidcraft_kernel::fillet(&pyramid, &edges, 1.5)
}
