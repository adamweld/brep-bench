//! The rod of radius 5 about the z axis from z = 0 to 20 less everything the bar 4 ≤ x ≤ 12, |y|
//! ≤ 8, |z| ≤ 1 passes through as it turns by θ about the z axis while rising 6s + 0.3s², s =
//! θ/360°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid, and its screw sweep keeps one lead: here the lead grows as the bar climbs, so no screw carries it and the twisted extrusion that would carry its level section has no counterpart in springrep".into())
}
