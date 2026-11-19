//! A pipe of radius 1 along three turns of the helix of radius 10 about the z axis, rising 5 a
//! turn, each end closed by a half ball: everything within 1 of the helix.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // The pipe as `sweep/helical-pipe`'s, which springrep cannot make.
    Err("springrep has no helical curve to sweep a section along (its screw sweep turns a \
         profile drawn in a plane through the axis, not one square to the path)"
        .into())
}
