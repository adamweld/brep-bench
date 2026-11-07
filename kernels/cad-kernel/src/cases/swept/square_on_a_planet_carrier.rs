//! The solid a square bar fills carried round on a planet: the prism from z = 0 to 2 on the square
//! of side 2 centred at (5, 0), its centre carried round the circle of radius 5 about z by t while
//! it turns about its own centre by −2t (by −3t against the carrier), t from 0° to 360°.
//!
//! Refused by the kernel. Two turns about parallel axes at different rates are no constant twist,
//! so the sweep takes the kernel's road for any other rigid motion (K2), which reads the tool as a
//! convex solid of revolution and refuses the bar, a box (`Unbuilt::Curved` at its corner).
use crate::sketch::block;
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

pub fn build() -> Result<Brep, String> {
    let bar = block([4., -1., 0.], [6., 1., 2.])?;
    let up = [0., 0., 1.];
    // on the carrier the bar turns by −3t about its own centre; the carrier turns by t about z.
    // The motion is the planet's turn seen from a frame turning by −t about z (`Relative`: at each
    // t the source's pose, then the observer's undone), so the carrier's turn is put back.
    let planet = Trajectory::Twist(Screw {
        origin: [5., 0., 0.], axis: up, ratio: -3., advance: 0.,
    });
    let carrier = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: up, ratio: -1., advance: 0.,
    });
    let path = Trajectory::Relative { source: Box::new(planet), observer: Box::new(carrier) };
    // no constant twist and no path of slides and turns: the kernel's road for any other rigid
    // motion (K2)
    closed(&bar, &path, [0., TAU], bar.size(), tolerance(bar.size()))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}
