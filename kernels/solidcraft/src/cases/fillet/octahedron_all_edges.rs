use crate::solids::*;
use solidcraft_kernel::HalfSpace;

pub fn build() -> R<Body> {
    // the octahedron |x| + |y| + |z| ≤ 10: its eight faces, (±1, ±1, ±1)/√3 · p ≤ 10/√3
    let s = 3f64.sqrt();
    let mut faces = Vec::new();
    for x in [-1.0, 1.0] {
        for y in [-1.0, 1.0] {
            for z in [-1.0, 1.0] {
                faces.push(HalfSpace { n: v3(x / s, y / s, z / s), d: 10.0 / s });
            }
        }
    }
    let octahedron = solidcraft_kernel::convex_polyhedron(&faces)?;
    // the midpoint of each of the twelve edges, between two corners on the axes
    let mut edges = Vec::new();
    for a in [-5.0, 5.0] {
        for b in [-5.0, 5.0] {
            edges.extend([v3(a, b, 0.0), v3(a, 0.0, b), v3(0.0, a, b)]);
        }
    }
    solidcraft_kernel::fillet(&octahedron, &edges, 2.0)
}
