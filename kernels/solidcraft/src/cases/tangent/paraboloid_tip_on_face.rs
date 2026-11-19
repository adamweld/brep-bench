use crate::solids::*;
use solidcore_record::Seg2;

/// The solid paraboloid ρ² ≤ `c`·z, z ≤ `top`, about the z axis: its half section in x and z
/// turned a whole turn. The parabola from the vertex to its rim is the quadratic Bézier it is, a
/// conic of weight 1 through its ends whose apex is where their tangents meet, (ρ/2, 0).
pub fn paraboloid(c: f64, top: f64) -> R<Body> {
    let rim = (c * top).sqrt();
    let outer = Loop2 {
        segs: vec![
            Seg2::Conic { a: Vec2::ZERO, apex: v2(rim / 2.0, 0.0), b: v2(rim, top), w: 1.0 },
            Seg2::Line { a: v2(rim, top), b: v2(0.0, top) },
            Seg2::Line { a: v2(0.0, top), b: Vec2::ZERO },
        ],
    };
    revolve_z(Region2 { outer, holes: vec![] }, 360.0)
}

pub fn build() -> R<Body> {
    union(&block((-10.0, -10.0, 0.0), (10.0, 10.0, 10.0))?, &paraboloid(8.0, 12.5)?)
}
