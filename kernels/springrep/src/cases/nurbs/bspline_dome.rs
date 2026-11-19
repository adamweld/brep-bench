//! A block on the square 0 ≤ x, y ≤ 20 under a clamped bicubic B-spline of 6 × 5 control points
//! with a double knot, its control points not at its knots' averages.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep makes no face from a B-spline surface's own control net: its only solid under \
         a free-form surface is primitive::slab, which stands each control point over its knots' \
         Greville abscissae (here x = 0, 2.67, 5.33, 12, 16, 20), not at the x = 0, 3, 7, 11, 16, \
         20 and y = 0, 5, 10, 15, 20 this surface's control points have, so a slab's top is \
         another surface"
        .into())
}
