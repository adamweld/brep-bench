//! A groove profiled round a six-lobed flower by a bull-nose mill in three step-downs. The plate is
//! the box from (0, 0, −16) to (120, 120, 0); a bull-nose mill of radius 4, its end corner rounded
//! to 1.5, 25 long, upright, plunges at (102, 60) and runs anticlockwise round the flower at
//! z = −4, −8 and −10, then rises. The flower is six lobes, arcs of radius 12 about the points 30
//! from (60, 60) at 0°, 60°, …, joined by six valleys, arcs about the points 40 from it at 30°,
//! 90°, …, each tangent to the lobes beside it.
//!
//! The mill stays upright and each move of a lap is an arc about an upright line, so what it sweeps
//! along an arc is its section through its axis (8 wide, its two bottom corners rounded to 1.5,
//! 25 high) turned about that line through the arc's angle: the kernel's revolve of that section,
//! its walls cylinders and planes, its corners tori. Each lap is its thirteen arcs united (they meet
//! face to face, a lobe and a valley sharing the plane through both centres where they touch); the
//! mill's positions where it plunges, rises and ends a lap stand at (102, 60), a point of the
//! flower, within the laps' grooves. The plate less the three, each cut in turn as the program cuts
//! them.
use crate::solids::p;
use crate::solids::swept::{planar, Piece};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_box_brep, revolve_profile_brep, BooleanOperation, BrepSolid};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// What the mill sweeps with its tip at height `z` along the arc of radius `radius` about the
/// upright line through `centre`, from the angle `from` anticlockwise through `sweep` (radians).
fn along_arc(centre: [f64; 2], radius: f64, from: f64, sweep: f64, z: f64) -> Result<BrepSolid, Refusal> {
    let (r, top) = (radius, z + 25.0);
    let section = [
        Piece::Line { from: [r - 2.5, z], to: [r + 2.5, z] },
        Piece::Arc { centre: [r + 2.5, z + 1.5], radius: 1.5, start: -FRAC_PI_2, end: 0.0 },
        Piece::Line { from: [r + 4.0, z + 1.5], to: [r + 4.0, top] },
        Piece::Line { from: [r + 4.0, top], to: [r - 4.0, top] },
        Piece::Line { from: [r - 4.0, top], to: [r - 4.0, z + 1.5] },
        Piece::Arc { centre: [r - 2.5, z + 1.5], radius: 1.5, start: PI, end: 1.5 * PI },
    ];
    let (o, up) = (p(centre[0], centre[1], 0.0), p(0.0, 0.0, 1.0));
    let curves = planar(&section, o, p(from.cos(), from.sin(), 0.0), up)?;
    Ok(revolve_profile_brep(&curves, o, up, sweep)?)
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut plate = make_box_brep(p(0.0, 0.0, -16.0), 120.0, 120.0, 16.0)?;
    let d = (2500.0 - 1200.0 * 3f64.sqrt()).sqrt();
    let at = |distance: f64, degrees: f64| {
        let a = degrees.to_radians();
        [60.0 + distance * a.cos(), 60.0 + distance * a.sin()]
    };
    let lobe = |k: usize| at(30.0, 60.0 * (k % 6) as f64);
    let valley = |k: usize| at(40.0, 30.0 + 60.0 * (k % 6) as f64);
    // The direction from one centre toward another: where the two arcs touch.
    let toward = |a: [f64; 2], b: [f64; 2]| (b[1] - a[1]).atan2(b[0] - a[0]);
    for z in [-4.0, -8.0, -10.0] {
        let mut arcs = Vec::new();
        for k in 0..6 {
            // Along the lobe anticlockwise, from its outermost point or where the valley before
            // left it, to where it touches its valley.
            let from = if k == 0 { 0.0 } else { toward(lobe(k), valley(k + 5)) };
            let to = toward(lobe(k), valley(k));
            arcs.push(along_arc(lobe(k), 12.0, from, (to - from).rem_euclid(TAU), z)?);
            // Round the valley clockwise to the next lobe: the same as anticlockwise from its end.
            let (from, to) = (toward(valley(k), lobe(k)), toward(valley(k), lobe(k + 1)));
            arcs.push(along_arc(valley(k), d - 12.0, to, (from - to).rem_euclid(TAU), z)?);
        }
        // Back along the lobe at 0° to its outermost point.
        let from = toward(lobe(0), valley(5));
        arcs.push(along_arc(lobe(0), 12.0, from, (0.0 - from).rem_euclid(TAU), z)?);
        let groove = boolean_operation_nary(&arcs, BooleanOperation::Union)?;
        plate = boolean_operation_nary(&[plate, groove], BooleanOperation::Subtract)?;
    }
    Ok(plate)
}
