//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) grooved by a torus
//! of radii 2.5 and 2 centred at (5, 0, 0), its axis leaning 25° from z towards +y, turned rigidly
//! about z from −140° to +140°.
//!
//! Built by the kernel's sweep of a ring cut from a blank (`swept::closed::ring_cut_from`): a torus
//! is a tool of revolution whose meridian is a loop clear of its axis, so its contact under the
//! turn is read meridian by meridian in closed form though its axis is skew to the turn's, and
//! each contact curve sweeps a tube, fitted. The sector is split by the tubes and the torus where
//! it starts and stops at once, and kept where the sweep is not: no closed solid is built and then
//! subtracted.
use crate::sketch::polygon;
use cad_kernel::build::{revolve, torus};
use cad_kernel::swept::closed::ring_cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const LEAN: f64 = 25f64.to_radians();
const TURN: f64 = 140f64.to_radians();
/// How closely the torus is meshed for the sweep's membership reads: a two-hundredth of its tube's
/// radius.
const BAR: f64 = 0.01;
/// The tolerance the tubes are fitted and met to: a two-hundred-thousandth of the tube's radius.
const TOL: f64 = 1e-5;

pub fn build() -> Result<Brep, String> {
    // the sector: its section in the xz plane turned 60° about z
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    let sector = revolve(&section, [0., 0., 0.], [0., 0., 1.], 60f64.to_radians())?;
    let cutter = torus([5., 0., 0.], [0., LEAN.sin(), LEAN.cos()], 2.5, 2.)?;
    // the turn about z: a screw that does not advance
    let path = Trajectory::Twist(Screw { origin: [0., 0., 0.], axis: [0., 0., 1.], ratio: 1., advance: 0. });
    ring_cut_from(&sector, &cutter, &path, [-TURN, TURN], BAR, TOL)
        .map_err(|why| format!("the cutter's sweep not built: {why:?}"))
}
