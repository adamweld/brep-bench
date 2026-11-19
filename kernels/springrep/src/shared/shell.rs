//! Helpers shared by the shell cases: the faces a shell leaves open, picked by name, and the solids
//! of revolution several cases start from, their meridian drawn as (distance from the z axis, z).
#![allow(dead_code)]

use std::f64::consts::TAU;

use springrep::edit::{Wall, shell};
use springrep::math::{Frame, Point3, Vec2, Vec3};
use springrep::pick::faces_named;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::{extrude, revolve};
use springrep::topo::{Brep, EdgeId, FaceId};

use crate::shared::e;

/// The faces carrying any of `names` (each `feature.face`); every name must pick at least one.
pub fn faces(b: &Brep, names: &[&str]) -> Result<Vec<FaceId>, String> {
    let mut out = Vec::new();
    for &name in names {
        let found = faces_named(b, name);
        if found.is_empty() {
            return Err(format!("no face is named {name}"));
        }
        out.extend(found);
    }
    Ok(out)
}

/// `b` shelled to a wall `thickness` thick on the `wall` side, open at the faces named `open`.
pub fn shelled(
    b: &Brep,
    open: &[&str],
    thickness: f64,
    wall: Wall,
    feature: &str,
) -> Result<Brep, String> {
    let open = faces(b, open)?;
    shell(b, &open, thickness, wall, feature).map_err(e)
}

/// The meridian `segments`, drawn in the half-plane with x the distance from the z axis and y the
/// height, turned by `angle` about the z axis (a whole turn when `angle` is `TAU`).
pub fn turned(segments: Vec<NamedSegment>, angle: f64, feature: &str) -> Result<Brep, String> {
    // the xz plane: the sketch's x along world x, its y along world z
    let plane = Frame::new(Point3::ZERO, -Vec3::Y, Vec3::X).map_err(e)?;
    let meridian = Profile::outline(segments);
    revolve(&meridian, &plane, Point3::ZERO, Vec3::Z, angle, feature).map_err(e)
}

/// A whole turn of the meridian `segments` about the z axis.
pub fn turned_whole(segments: Vec<NamedSegment>, feature: &str) -> Result<Brep, String> {
    turned(segments, TAU, feature)
}

/// The straight meridian edge from `a` to `b`, both (distance from the axis, z).
pub fn line(a: [f64; 2], b: [f64; 2], name: &str) -> NamedSegment {
    NamedSegment::line(Vec2::new(a[0], a[1]), Vec2::new(b[0], b[1]), name)
}

/// The meridian arc about `center` of `radius`, from angle `start` through `sweep` (radians,
/// counter-clockwise when positive).
pub fn arc(center: [f64; 2], radius: f64, start: f64, sweep: f64, name: &str) -> NamedSegment {
    NamedSegment::arc(Vec2::new(center[0], center[1]), radius, start, sweep, name)
}

/// The edges of `b` whose two ends both pass `on`, a test of a point.
pub fn edges_where(b: &Brep, on: impl Fn(Point3) -> bool) -> Vec<EdgeId> {
    b.edge_ids()
        .filter(|&id| {
            let edge = b.edge(id);
            on(b.vertex(edge.start).point) && on(b.vertex(edge.end).point)
        })
        .collect()
}

/// The prism of the polygon through `points` in the xy plane, edge `i` (from point `i` to point
/// `i + 1`) named `names[i]`, extruded `height` along +z from z = 0.
pub fn prism(
    points: &[[f64; 2]],
    names: &[&str],
    height: f64,
    feature: &str,
) -> Result<Brep, String> {
    let points: Vec<Vec2> = points.iter().map(|p| Vec2::new(p[0], p[1])).collect();
    let outline = Profile::polygon(&points, names);
    extrude(
        &outline,
        &Frame::new(Point3::ZERO, Vec3::Z, Vec3::X).map_err(e)?,
        height,
        feature,
    )
    .map_err(e)
}
