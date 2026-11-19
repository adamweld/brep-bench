//! The rod of radius 5 about the z axis from z = 0 to 20 less everything an oblate spheroid
//! wheel (semi-axes 6, 6, 1.5 about (10.5, 0, 0), leaning 44.83°) passes through as it is
//! screwed about the z axis, rising 30 a turn.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The wheel is set to the helix, so under the screw it touches what it sweeps along a curve over its ellipsoidal side: the groove is an envelope springrep cannot make from its profile sweeps".into())
}
