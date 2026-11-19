//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a torus wheel (radii 6
//! and 2 about (11.5, 0, 0), leaning 49.04°) passes through as it is screwed about the z axis,
//! rising 30 a turn.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The wheel is set to the helix, so under the screw it touches what it sweeps along a curve over its tube: the groove is an envelope springrep cannot make from its profile sweeps".into())
}
