//! The pyramid on the rectangle from (−10, −6, 0) to (10, 6, 0) with its apex at (0, 0, 12), all
//! eight edges rounded to radius 3/2 in one operation.
use springrep::blend::fillet_edges;
use springrep::boolean::intersection;
use springrep::math::Vec3;
use springrep::pick::convex_edges;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::chamfer::half_space;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the block over the base, cut by the half-space under each sloping face: the plane through a
    // base edge and the apex
    let apex = Vec3::new(0., 0., 12.);
    let mut solid = block(&at([-10., -6., 0.]), Vec3::new(20., 12., 20.), "base").map_err(e)?;
    let edges = [
        (Vec3::new(10., -6., 0.), Vec3::new(10., 6., 0.)),
        (Vec3::new(10., 6., 0.), Vec3::new(-10., 6., 0.)),
        (Vec3::new(-10., 6., 0.), Vec3::new(-10., -6., 0.)),
        (Vec3::new(-10., -6., 0.), Vec3::new(10., -6., 0.)),
    ];
    for (i, (a, b)) in edges.into_iter().enumerate() {
        // the base runs counter-clockwise seen from above, so (b − a) × (apex − a) points out
        let outward = (b - a).cross(apex - a);
        let side = half_space(a, -outward, 80., &format!("side{i}"))?;
        solid = intersection(&solid, &side).map_err(e)?;
    }
    let edges = convex_edges(&solid).map_err(e)?;
    fillet_edges(&solid, &edges, 1.5, "round").map_err(e)
}
