//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a drafted bar of slot
//! section pointing at the axis passes through as it is screwed about the z axis, rising 6 a
//! turn.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The bar's drafted sides and half cones lean to the screw's axis, so they touch what they sweep along curves off its section through the axis, and its level sections change along the axis: the groove's walls are envelopes springrep cannot make from its profile sweeps".into())
}
