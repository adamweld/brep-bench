//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a V wheel (a sharp rim
//! of radius 10 about the line through (14, 0, 10) leaning 20° across the turn) passes through
//! as it turns once about the z axis.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. A whole turn sweeps the wheel's shadow on a half-plane through the axis, turned; leaning, its flanks touch along curves that are none of its circles, so that shadow's outline is a curve springrep would have to compute".into())
}
