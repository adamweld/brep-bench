//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a ball of radius 1
//! passes through as its center runs six turns of the helix of radius 5 rising 4 a turn, from
//! (5, 0, −2).
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid, and no helical curve: a ball's sweep along a helix is the tube about the helix, which its path sweep could carry the ball's circle along only on a helix it cannot make, and its screw sweep turns a section through the axis, which for a tube is the tube's own section there, an envelope it would have to compute".into())
}
