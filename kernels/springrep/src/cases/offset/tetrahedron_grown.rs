//! The regular tetrahedron with corners (3, 3, 3), (3, −3, −3), (−3, 3, −3) and (−3, −3, 3), grown
//! by 1. springrep has no polyhedron, so the tetrahedron is a block cut by the planes of its four
//! faces, each keeping the side its opposite corner is on.
use springrep::boolean::split;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let mut tet = block(&at([-5., -5., -5.]), Vec3::new(10., 10., 10.), "block").map_err(e)?;
    let corners = [[3., 3., 3.], [3., -3., -3.], [-3., 3., -3.], [-3., -3., 3.]];
    for corner in corners {
        // the face opposite a corner passes through the other three, a third of the way from the
        // centre to the corner, back across the centre: its normal points at the corner
        let toward = Vec3::from_array(corner);
        let (kept, _) = split(&tet, toward * (-1. / 3.), toward).map_err(e)?;
        tet = kept;
    }
    grown(&tet, 1.)
}
