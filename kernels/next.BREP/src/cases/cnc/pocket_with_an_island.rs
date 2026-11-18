//! A 2.5-axis program of three tools on the plate from (0, 0, −20) to (160, 110, 0): a pocket round
//! a round island cleared at z = −4, −8 and −12 by a flat end mill of radius 5 (raster passes along
//! y = 26, 30, …, 82 between the wall path, the rectangle (25, 25)–(135, 85) with its corners
//! rounded to 7, and the island path, the circle of radius 20 about (60, 55); then once round each
//! path), a bore at the island's top by a flat end mill of radius 3 (whole circles of radius 6 about
//! (60, 55) at z = −3 and −6), and four holes pecked to z = −14 by a drill of radius 4 with a 90°
//! point. Every move between cuts is made at z = 5. The solid is the plate less everything the
//! cutters pass through, each move swept by the kernel's sweep of a solid and cut in turn (passing
//! over a move a deeper one of the same cutter covers within the plate: `cut_in_turn`).
use crate::solids::cnc::{cut_in_turn, rounded_loop, Cutter, Program};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};
use std::f64::consts::FRAC_PI_4;

const CLEAR: f64 = 5.0;

/// The wall path: the rectangle (25, 25)–(135, 85), its corners rounded to 7.
const WALL: [[f64; 2]; 4] = [[25.0, 25.0], [135.0, 25.0], [135.0, 85.0], [25.0, 85.0]];
const WALL_RADIUS: f64 = 7.0;
const ISLAND: [f64; 2] = [60.0, 55.0];
const ISLAND_RADIUS: f64 = 20.0;

/// Where the line y = `y` crosses the wall path: its left and right ends.
fn across_wall(y: f64) -> (f64, f64) {
    let (x0, x1, y0, y1, r) = (25.0, 135.0, 25.0, 85.0, WALL_RADIUS);
    let inset = if y < y0 + r {
        r - (r * r - (y0 + r - y).powi(2)).sqrt()
    } else if y > y1 - r {
        r - (r * r - (y - (y1 - r)).powi(2)).sqrt()
    } else {
        0.0
    };
    (x0 + inset, x1 - inset)
}

/// The intervals of the line y = `y` inside the wall path and outside the island path, left to
/// right.
fn intervals(y: f64) -> Vec<(f64, f64)> {
    let (left, right) = across_wall(y);
    let dy = y - ISLAND[1];
    if dy.abs() >= ISLAND_RADIUS {
        return vec![(left, right)];
    }
    let half = (ISLAND_RADIUS * ISLAND_RADIUS - dy * dy).sqrt();
    vec![(left, ISLAND[0] - half), (ISLAND[0] + half, right)]
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, 0.0, -20.0), 160.0, 110.0, 20.0)?;

    let mut pocket = Program::new(Cutter::flat(5.0, 30.0), p(80.0, 55.0, CLEAR), 0.0);
    for z in [-4.0, -8.0, -12.0] {
        for k in 0..=14 {
            let y = 26.0 + 4.0 * k as f64;
            for (from, to) in intervals(y) {
                pocket.line(p(from, y, CLEAR))?;
                pocket.line(p(from, y, z))?;
                pocket.line(p(to, y, z))?;
                pocket.line(p(to, y, CLEAR))?;
            }
        }
        pocket.line(p(80.0, 25.0, CLEAR))?;
        pocket.line(p(80.0, 25.0, z))?;
        rounded_loop(&mut pocket, &WALL, WALL_RADIUS, z)?;
        pocket.line(p(80.0, 25.0, CLEAR))?;
        pocket.line(p(80.0, 55.0, CLEAR))?;
        pocket.line(p(80.0, 55.0, z))?;
        pocket.arc(ISLAND, p(80.0, 55.0, z), false)?;
        pocket.line(p(80.0, 55.0, CLEAR))?;
    }

    let mut bore = Program::new(Cutter::flat(3.0, 30.0), p(60.0, 55.0, CLEAR), 0.0);
    for z in [-3.0, -6.0] {
        bore.line(p(60.0, 55.0, z))?;
        bore.line(p(66.0, 55.0, z))?;
        bore.arc(ISLAND, p(66.0, 55.0, z), true)?;
        bore.line(p(60.0, 55.0, z))?;
    }
    bore.line(p(60.0, 55.0, CLEAR))?;

    let mut drill = Program::new(Cutter::drill(4.0, FRAC_PI_4, 30.0), p(8.0, 8.0, CLEAR), 0.0);
    for [x, y] in [[8.0, 8.0], [152.0, 8.0], [152.0, 102.0], [8.0, 102.0]] {
        drill.line(p(x, y, CLEAR))?;
        for z in [-5.0, 2.0, -10.0, 2.0, -14.0, CLEAR] {
            drill.line(p(x, y, z))?;
        }
    }

    cut_in_turn(plate, 0.0, vec![pocket, bore, drill])
}
