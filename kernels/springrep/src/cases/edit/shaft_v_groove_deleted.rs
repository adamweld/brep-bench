//! A shaft of radius 5 from z = 0 to z = 20 with a V-groove, two cones meeting at radius 3 at
//! z = 10; both cones then deleted together, the cylinder running on across the groove.
use std::f64::consts::TAU;

use springrep::boolean::difference;
use springrep::math::{Point3, Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::edit::{delete_named, shaft};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the tool's meridian, x out from the axis and y up it: a V with its point at radius 3, z = 10
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let v = Profile::outline(vec![
        NamedSegment::line(p(3., 10.), p(7., 6.), "lower"),
        NamedSegment::line(p(7., 6.), p(7., 14.), "outer"),
        NamedSegment::line(p(7., 14.), p(3., 10.), "upper"),
    ]);
    let plane = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    let tool = revolve(&v, &plane, Point3::ZERO, Vec3::Z, TAU, "v").map_err(e)?;
    let grooved = difference(&shaft()?, &tool).map_err(e)?;
    delete_named(&grooved, &["v.lower", "v.upper"])
}
