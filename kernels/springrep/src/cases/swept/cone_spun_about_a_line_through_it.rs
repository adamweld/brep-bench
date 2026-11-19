//! The solid a cone (axis along (cos 30°, 0, sin 30°), point at −2 along it, opening at 50° to a
//! base at 5) fills as it turns 150° about the z axis.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The cone's axis is oblique to the turn's, so its side touches what it sweeps along curves off any half-plane through the turn's axis: the swept solid's walls are envelopes springrep cannot make from its profile sweeps".into())
}
