//! A dome machined by a five-axis program of four cutters. The blank is the cylinder of radius 50
//! about the z axis from z = 0 to 45; the dome aimed at, the ball of radius 40 about the origin.
//! A bull-nose mill (radius 8, corner 2) roughs it upright in eighteen level circles; a ball-end
//! mill of radius 4, held square to the dome, finishes it along 180 meridians (latitude 3° to
//! 75°, 2° apart), round 15 circles near the pole and with a touch at the pole; a flat end mill
//! of radius 5 faces twelve seats at latitude 40°, and a drill of radius 2.5 drills each, both
//! square to the dome.
//!
//! The program repeats itself round the z axis, and the dome is built as it does: one sector
//! of it machined, and the sector patterned round.
//!
//! - The roughing circles, the circles round the pole and the touch at the pole are whole turns
//!   about the z axis: what they leave is the same at every longitude.
//! - The meridian passes are one pass turned on by 2° each time, and each is its own mirror
//!   image in its meridian plane. Within the blank a pass's cutter is a ball of radius 4 rolled
//!   along its axis, the axis always in the pass's meridian half plane; a ball centred in that
//!   half plane covers, of any circle about the z axis, an arc with its middle on the meridian.
//!   So of that circle the passes together cover what the nearest pass covers alone: within 1°
//!   of a pass's meridian the dome is the blank less that pass, whatever its neighbours do.
//! - The seats and holes repeat every 30°, each within the 30° from 1° before one meridian pass
//!   to 1° before the fifteenth after it.
//!
//! So: the 1° of the blank from a pass's meridian plane to the plane halfway to the next pass is
//! machined by the roughing, that pass, the circles and the touch; its mirror image in the
//! meridian plane makes the pass's own 2°; fifteen of those, each turned 2° on, make 30°, in
//! which one seat is faced and one hole drilled; and twelve of those make the dome. Every sweep
//! is cadrs's own: a roughing circle's groove is the mill's section through its axis carried
//! upright round the circle (Sweep along a path), and what a leaning cutter's slide or turn
//! sweeps is the sweep of a solid, the cutter put square to the dome by moving it up the z axis,
//! turning it down to its latitude and round to its longitude. The moves between passes, clear
//! of the blank, cut nothing and are left out.
use crate::bench::{arc, boolean, cut, cylinder, mirrored, moved, p2, p3, revolve, turned, union, v3, V3};
use crate::shared::cnc::{across, round_the_path};
use crate::shared::geometry::Section;
use crate::shared::swept::{slid, swung, z_axis};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, BoolOp, DefaultKernel, Plane};

const DOME: f64 = 40.0;

/// The dome's outward normal at a latitude and a longitude (degrees).
fn normal(lat: f64, lon: f64) -> V3 {
    let (l, p) = (lat.to_radians(), lon.to_radians());
    v3(l.cos() * p.cos(), l.cos() * p.sin(), l.sin())
}

/// `cutter` (upright, its tip at the origin) square to the dome at a latitude and a longitude,
/// its tip `r` from the origin: moved up the z axis, turned down about the y axis, then round
/// the z axis.
fn square_to(k: &mut DefaultKernel, cutter: BodyId, lat: f64, lon: f64, r: f64) -> Result<BodyId, Refusal> {
    let up = moved(k, cutter, v3(0.0, 0.0, r))?;
    let leaning = turned(k, up, Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 1.0, 0.0)), 90.0 - lat)?;
    turned(k, leaning, z_axis(), lon)
}

/// A cutter's half section turned about the z axis.
fn turned_section(k: &mut DefaultKernel, half: Section) -> Result<BodyId, Refusal> {
    revolve(k, &half.sketch(), z_axis(), None)
}

/// `count` copies of `part` round the z axis, `step` degrees apart, as one body.
fn round_the_axis(k: &mut DefaultKernel, part: BodyId, count: usize, step: f64) -> Result<BodyId, Refusal> {
    let mut copies = Vec::new();
    for i in 1..count {
        copies.push(turned(k, part, z_axis(), step * i as f64)?);
    }
    boolean(k, BoolOp::Union, part, &copies)
}

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let up = v3(0.0, 0.0, 1.0);
    let w = 2f64.sqrt() / 2.0;
    // The blank from the first meridian pass's plane (y = 0) to the plane halfway to the next
    // pass, 1° on: its half section turned 1° about the z axis.
    let half_section = Section::on(Plane::xz(0.0))
        .line((0.0, 0.0), (50.0, 0.0))
        .line((50.0, 0.0), (50.0, 45.0))
        .line((50.0, 45.0), (0.0, 45.0))
        .line((0.0, 45.0), (0.0, 0.0));
    let mut part = revolve(k, &half_section.sketch(), z_axis(), Some(1.0))?;
    // 1. Roughing: the bull-nose mill, upright, in level circles. Each circle's groove is the
    // mill's section through its axis carried upright round the circle (the plunge and the rise
    // at (R, 0), a point of the circle, are within it).
    for z in [41.0f64, 36.0, 31.0, 26.0, 21.0, 16.0, 11.0, 6.0] {
        let rho = (DOME * DOME - z * z).max(0.0).sqrt();
        let mut radii = Vec::new();
        let mut r = rho + 9.0;
        while r < 58.0 {
            radii.push(r);
            r += 12.0;
        }
        for r in radii.into_iter().rev() {
            let section = Section::on(across(r, 0.0, (0.0, 1.0)))
                .line((-6.0, z), (6.0, z))
                .conic((6.0, z), (8.0, z), w, (8.0, z + 2.0))
                .line((8.0, z + 2.0), (8.0, 50.0))
                .line((8.0, 50.0), (-8.0, 50.0))
                .line((-8.0, 50.0), (-8.0, z + 2.0))
                .conic((-8.0, z + 2.0), (-8.0, z), w, (-6.0, z));
            let ring = round_the_path(
                k,
                &section.sketch(),
                vec![arc(p2(0.0, 0.0), r, 0.0, 180.0), arc(p2(0.0, 0.0), r, 180.0, 180.0)],
            )?;
            part = cut(k, part, ring)?;
        }
    }
    // 2. Finishing: the ball-end mill square to the dome.
    let ball = turned_section(
        k,
        Section::on(Plane::xz(0.0))
            .conic((0.0, 0.0), (4.0, 0.0), w, (4.0, 4.0))
            .line((4.0, 4.0), (4.0, 40.0))
            .line((4.0, 40.0), (0.0, 40.0))
            .line((0.0, 40.0), (0.0, 0.0)),
    )?;
    let reach = 90.0 - DOME;
    // The meridian pass at longitude 0: in along its axis at latitude 3°, up the meridian to
    // 75° (a turn about the level line square to it), out along its axis.
    let low = square_to(k, ball, 3.0, 0.0, DOME)?;
    let slide_in = slid(k, low, normal(3.0, 0.0) * reach)?;
    part = cut(k, part, slide_in)?;
    let level = Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, -1.0, 0.0));
    let climb = swung(k, low, level, 0.0, 72.0)?;
    part = cut(k, part, climb)?;
    let high = square_to(k, ball, 75.0, 0.0, DOME)?;
    let slide_out = slid(k, high, normal(75.0, 0.0) * reach)?;
    part = cut(k, part, slide_out)?;
    // Round the pole: in along its axis, once round the z axis, out again the way it came.
    for lat in 75..90 {
        let at = square_to(k, ball, lat as f64, 0.0, DOME)?;
        let slide = slid(k, at, normal(lat as f64, 0.0) * reach)?;
        part = cut(k, part, slide)?;
        let circle = swung(k, at, z_axis(), 0.0, 360.0)?;
        part = cut(k, part, circle)?;
    }
    let pole = square_to(k, ball, 90.0, 0.0, DOME)?;
    let touch = slid(k, pole, up * reach)?;
    part = cut(k, part, touch)?;
    // The pass's own 2° of the dome, and fifteen passes' 30°, from −1° to 29°.
    let other = mirrored(k, part, p3(0.0, 0.0, 0.0), v3(0.0, 1.0, 0.0))?;
    let slice = union(k, part, other)?;
    let mut sector = round_the_axis(k, slice, 15, 2.0)?;
    // 3. The seat and 4. the hole at longitude 15°, each cutter slid in along the normal and
    // back.
    let flat = cylinder(k, p3(0.0, 0.0, 0.0), up, 5.0, 40.0)?;
    let drill = turned_section(
        k,
        Section::on(Plane::xz(0.0))
            .line((0.0, 0.0), (2.5, 2.5))
            .line((2.5, 2.5), (2.5, 40.0))
            .line((2.5, 40.0), (0.0, 40.0))
            .line((0.0, 40.0), (0.0, 0.0)),
    )?;
    for (cutter, depth) in [(flat, 38.5), (drill, 30.0)] {
        let at = square_to(k, cutter, 40.0, 15.0, depth)?;
        let plunge = slid(k, at, normal(40.0, 15.0) * (70.0 - depth))?;
        sector = cut(k, sector, plunge)?;
    }
    round_the_axis(k, sector, 12, 30.0)
}
