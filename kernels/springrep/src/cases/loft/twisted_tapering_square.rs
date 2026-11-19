//! A square column from z = 0 to 40 of side 30 − z/2, turned 4.5° per unit of height about the
//! z axis: from the square of side 30 to the square of side 10, half a turn on.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // Its walls are ruled between spiral guides; springrep lofts through sections only, and a
    // smooth loft through many sampled sections would only approximate them.
    Err("springrep has no loft along guide curves, nor a twisted taper (its screw sweep \
         keeps the section's size)"
        .into())
}
