//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a frustum
//! about the upright line through (10, 0) (radius 0.4 at z = −0.6 to 1 at z = 1.4) passes
//! through as it is screwed about the z axis, rising 2 a turn, from −140° to 140°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The frustum's side leans to the screw's axis, so under the screw it touches what it sweeps along lines off the plane through both axes (where the side's slope meets the screw's rise): the groove's walls are envelopes springrep cannot make from its profile sweeps".into())
}
