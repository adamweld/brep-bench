//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a V wheel (a sharp rim
//! of radius 10 about (14, 0, 0), leaning 43.30°) passes through as it is screwed about the z
//! axis, rising 30 a turn.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The wheel is set to the helix, so under the screw its flanks touch what they sweep along curves off its section through the axis: the groove is an envelope springrep cannot make from its profile sweeps".into())
}
