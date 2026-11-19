//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a leaning
//! bar, its end a 3 × 4 rectangle turned 5° in its plane, passes through as it turns about the z
//! axis from −140° to 140°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The bar's section is turned so no side faces the axis squarely: its inner side touches what it sweeps along a line crossing its rulings, and its rim carries the contact from side to end, so the cut's walls are envelopes springrep cannot make from its profile sweeps".into())
}
