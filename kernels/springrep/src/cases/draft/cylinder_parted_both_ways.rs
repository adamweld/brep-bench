//! A cylinder of radius 10 from z = 0 to z = 20, its side parted at z = 10 and each half drafted
//! 3° away from the parting line: two frustums joined at their wide ends.
use std::f64::consts::TAU;

use springrep::edit::draft;
use springrep::math::{Point3, Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::draft::carrying;
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the meridian, drawn with x out from the axis and y up it, its side in two segments that meet
    // at the parting line, so the turned side is two faces, `lower` and `upper`
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let meridian = Profile::outline(vec![
        NamedSegment::line(p(0., 0.), p(10., 0.), "bottom"),
        NamedSegment::line(p(10., 0.), p(10., 10.), "lower"),
        NamedSegment::line(p(10., 10.), p(10., 20.), "upper"),
        NamedSegment::line(p(10., 20.), p(0., 20.), "top"),
        NamedSegment::line(p(0., 20.), p(0., 0.), "axis"),
    ]);
    let plane = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    let post = revolve(&meridian, &plane, Point3::ZERO, Vec3::Z, TAU, "post").map_err(e)?;
    // the upper half pulled along +z and the lower along −z, both about the parting plane z = 10;
    // the halves share one cylinder, so each is picked by its own name only
    let upper = carrying(&post, "post.upper")?;
    let up = frame([0., 0., 10.], [0., 0., 1.], [1., 0., 0.])?;
    let post = draft(&post, &upper, &up, 3f64.to_radians()).map_err(e)?;
    let lower = carrying(&post, "post.lower")?;
    let down = frame([0., 0., 10.], [0., 0., -1.], [1., 0., 0.])?;
    draft(&post, &lower, &down, 3f64.to_radians()).map_err(e)
}
