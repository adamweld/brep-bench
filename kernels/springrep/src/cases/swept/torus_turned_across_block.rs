//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a torus
//! (radii 2.5 and 2 about (5, 0, 0), its axis leaning 25° toward +y) passes through as it turns
//! about the z axis from −140° to 140°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The torus's axis is skew to the turn's, so it touches what it sweeps along curves that are none of its circles: the groove's wall is an envelope springrep cannot make from its profile sweeps".into())
}
