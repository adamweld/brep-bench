//! The box from the origin to (20, 10, 10) with a slot 4 wide cut into its side y = 0, top to
//! bottom, closed by a half cylinder about (10, 5); the slot's three faces then deleted.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::math::{Vec2, Vec3};
use springrep::primitive::block;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 10., 10.), "block").map_err(e)?;
    // the cutter's outline: in from beyond the side y = 0 between x = 8 and x = 12, round at y = 5
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let slot = Profile::outline(vec![
        NamedSegment::line(p(8., -1.), p(12., -1.), "mouth"),
        NamedSegment::line(p(12., -1.), p(12., 5.), "right"),
        NamedSegment::arc(p(10., 5.), 2., 0., PI, "end"),
        NamedSegment::line(p(8., 5.), p(8., -1.), "left"),
    ]);
    let cutter = extrude(&slot, &at([0., 0., -1.]), 12., "slot").map_err(e)?;
    let slotted = difference(&b, &cutter).map_err(e)?;
    delete_named(&slotted, &["slot.left", "slot.right", "slot.end"])
}
