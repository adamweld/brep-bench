//! The Steinmetz solid common to two cylinders of radius 3 and length 20 centred on
//! c = (600000, −800000, 0), one along (2, 3, 6)/7 and one along (3, −6, 2)/7, each running 10
//! either way from c: their axes meet square at c, along directions no axis lines up with.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::boolean::rod;
use crate::shared::e;

const CENTRE: [f64; 3] = [600000., -800000., 0.];

/// The cylinder of radius 3 along the unit `axis` from c − 10·axis to c + 10·axis.
fn crossing(axis: [f64; 3], name: &str) -> Result<Brep, String> {
    let base = [0, 1, 2].map(|i| CENTRE[i] - 10. * axis[i]);
    rod(base, axis, 3., 20., name)
}

pub fn build() -> Result<Brep, String> {
    let u = [2. / 7., 3. / 7., 6. / 7.];
    let v = [3. / 7., -6. / 7., 2. / 7.];
    intersection(&crossing(u, "along_u")?, &crossing(v, "along_v")?).map_err(e)
}
