//! A transition from the square −20 ≤ x, y ≤ 20 at z = 0 to the circle of radius 15 at z = 30:
//! four flat triangles and four pieces of oblique cones, the hull of the two.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep's loft pairs a square with a circle by splitting each at the other's vertices' \
         polar angles into ruled twisted sides, not a corner joined to a quarter circle, and it \
         lofts to no point; it has no oblique cone (primitive::cone is a right circular one about \
         its frame's axis), so the hull's conical walls cannot be built"
        .into())
}
