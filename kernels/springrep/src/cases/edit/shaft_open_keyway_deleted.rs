//! A shaft of radius 5 from z = 0 to z = 20 with an open keyway 2 wide, its floor at y = −3, from
//! z = 8 out through the top; the keyway's four faces then deleted.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::edit::{delete_named, shaft};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let key = block(&at([-1., -6., 8.]), Vec3::new(2., 3., 13.), "key").map_err(e)?;
    let cut = difference(&shaft()?, &key).map_err(e)?;
    // its walls x = ∓1, its floor y = −3 and its end z = 8
    delete_named(&cut, &["key.u_min", "key.u_max", "key.v_max", "key.start"])
}
