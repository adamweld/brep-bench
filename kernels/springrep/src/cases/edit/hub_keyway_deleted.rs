//! A hub, the tube of outer radius 10 and bore 4 from z = 0 to z = 20, with a keyway 2 wide cut
//! along its bore to y = −5; the keyway's three faces then deleted, giving the plain hub back.
use springrep::boolean::difference_all;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let hub = cylinder(&at([0., 0., 0.]), 10., 20., "hub").map_err(e)?;
    let bore = cylinder(&at([0., 0., -1.]), 4., 22., "bore").map_err(e)?;
    let key = block(&at([-1., -5., -1.]), Vec3::new(2., 5., 22.), "key").map_err(e)?;
    let part = difference_all(&hub, &[&bore, &key]).map_err(e)?;
    // the keyway's walls x = ∓1 and its floor y = −5
    delete_named(&part, &["key.u_min", "key.u_max", "key.v_min"]).map_err(|why| {
        format!("{why} (the bore's wall, hollow: springrep closes a gap only against planes and cylinders bulging outward)")
    })
}
