//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a cone pointed at (4,
//! 0, 0) along the x axis, opening at 60°, passes through as it is screwed about the z axis,
//! rising 6 a turn.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. Under the screw the cone's side touches what it sweeps along lines off the plane through the screw's axis, so the groove's flanks are envelopes of the cone, which springrep cannot make from its profile sweeps".into())
}
