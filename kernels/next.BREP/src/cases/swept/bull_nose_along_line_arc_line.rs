//! The block from (−10, −10, −10) to (40, 40, 0), less everything a bull-nose mill passes through
//! as its upright axis slides from (0, 0) to (20, 0), turns a quarter turn about the upright line
//! through (20, 10) to (30, 10), and slides on to (30, 30). The mill, about its axis: a flat bottom
//! of radius 1.5 at z = −1.5, a quarter torus of tube radius 0.5 about the circle of radius 1.5 at
//! z = −1, and a side of radius 2 up to z = 5.
//!
//! next.BREP's sweep of a solid builds a turn of a solid of revolution only where its ends stay
//! clear of the region asked for, and takes no slide of one, nor a path of several pieces. What a
//! path of pieces sweeps is the union of what each piece sweeps, and each follows by elementary
//! geometry: an upright solid of revolution slid along a level line sweeps the tool at each end and
//! its section through its axis, square to the slide, slid between them
//! (`solids::swept::upright_slid`); turned about a parallel upright line it sweeps the tool at each
//! end and its section by the half plane through both axes turned between them
//! (`solids::swept::upright_turned`, here about (20, 10) from −90° to 0°). All of it is cut from
//! the block.
use crate::solids::p;
use crate::solids::sweep::moved;
use crate::solids::swept::{slide, upright_slid, upright_turned, Meridian, Piece};
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-10.0, -10.0, -10.0), 50.0, 50.0, 10.0)?;
    let mill = Meridian(vec![
        Piece::Line { from: [0.0, -1.5], to: [1.5, -1.5] },
        Piece::Arc { centre: [1.5, -1.0], radius: 0.5, start: -FRAC_PI_2, end: 0.0 },
        Piece::Line { from: [2.0, -1.0], to: [2.0, 5.0] },
        Piece::Line { from: [2.0, 5.0], to: [0.0, 5.0] },
    ]);
    // each piece's sweep; the tool where one piece stops and the next starts is kept once
    let mut operands = upright_slid(&mill, p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0))?;
    let turned = upright_turned(&mill, 10.0, -FRAC_PI_2, 0.0)?;
    operands.push(moved(&turned[1], slide(p(20.0, 10.0, 0.0))?)?);
    operands.extend(upright_slid(&mill, p(30.0, 10.0, 0.0), p(0.0, 20.0, 0.0))?);
    let mut left = block;
    for piece in &operands {
        left = crate::solids::fillet::subtract(&left, piece)?;
    }
    Ok(left)
}
