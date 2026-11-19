//! The plate (−15, −15, 0)–(15, 15, 4) less everything a coil spring (a ball of radius 0.4 swept
//! three turns along the helix of radius 2 rising 1.5 a turn) passes through as it slides along
//! (3, 1, 18).
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid, and no helical curve: a ball's sweep along a helix is the tube about the helix, which its path sweep could carry the ball's circle along only on a helix it cannot make, and its screw sweep turns a section through the axis, which for a tube is the tube's own section there, an envelope it would have to compute; and the spring's shadow along the push is bounded by its coils' silhouettes".into())
}
