//! A pipe of radius 1 along three right-handed turns of the helix of radius 10 about the z axis,
//! rising 5 a turn from (10, 0, 0), its section square to the path.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // `sweep::helix` screws a profile drawn in a plane through the axis, a section square to the
    // path being no such profile; and springrep has no helical curve to sweep along: a path
    // through points sampled on the helix would be geometry made outside the kernel.
    Err("springrep has no helical curve to sweep a section along (its screw sweep turns a \
         profile drawn in a plane through the axis, not one square to the path)"
        .into())
}
