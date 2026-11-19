//! The ring sector (radii 8 to 12, heights −2 to 2, angles −30° to 30°) less everything a ball-
//! end mill (radius 1 about the upright line through (10, 0), its ball centered at (10, 0, 0))
//! passes through as it is screwed about the z axis, rising 3 a turn, from −60° to 60°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid, and no helical curve: a ball's sweep along a helix is the tube about the helix, which its path sweep could carry the ball's circle along only on a helix it cannot make, and its screw sweep turns a section through the axis, which for a tube is the tube's own section there, an envelope it would have to compute; the mill's ball sweeps the tube about its center's helix".into())
}
