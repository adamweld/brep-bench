//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a square thread cut
//! by the bar 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1 carried along the screw about the z axis that
//! rises 6 a turn, from a turn of −90° to one of 1350°.
//!
//! solidcraft-kernel has no sweep of a solid, so the bar's sweep is built by elementary geometry
//! from the kernel's helical sweep of planar regions (`sweep_helix`), which is the set a region
//! passes through wherever the screw carries it across itself, never along it. Split the bar by
//! the plane y = 0, through the screw's axis. The half on y ≥ 0 moves away from a point only
//! through its face z = −1, its back x = 12 and the cut y = 0 (the screw turns from +x toward +y),
//! so a point it passes over and leaves before the end is left through one of those; the half on
//! y ≤ 0 likewise first reaches a point through its face z = 1, its back or the cut. So the bar's
//! sweep is the bar where it starts and where it ends, with the screw sweeps of its section by
//! y = 0, its faces z = ±1 and its back. The back stays 12 from the axis and the two places of the
//! bar lie wholly below and above the rod, so the section and the faces are what it cuts.
use crate::bench::{rotated, translated};
use crate::shared::sweep::SCREW_TOLERANCE;
use crate::solids::*;
use crate::Refusal;
use solidcore_ops::sweep::{screw_sweep, Curve};
use solidcore_ops::Body as OpsBody;
use solidcraft_kernel::sweep_helix;

/// `p` turned by `deg` degrees about the z axis, from +x toward +y.
pub fn turned(p: Vec3, deg: f64) -> Vec3 {
    let (s, c) = deg.to_radians().sin_cos();
    v3(c * p.x - s * p.y, s * p.x + c * p.y, p.z)
}

/// `plane` carried to its turn of `deg` degrees along the screw about the z axis that rises
/// `lead` a turn.
pub fn screwed(plane: &Plane, lead: f64, deg: f64) -> Plane {
    let rise = v3(0.0, 0.0, lead * deg / 360.0);
    Plane { origin: turned(plane.origin, deg) + rise, x: turned(plane.x, deg), y: turned(plane.y, deg) }
}

/// What `region` on `plane` passes through as the screw about the z axis that rises `lead` a
/// turn carries it from its turn of `start` degrees to that of `stop`.
pub fn screw(plane: &Plane, region: &Region2, lead: f64, start: f64, stop: f64) -> R<Body> {
    let at = screwed(plane, lead, start);
    sweep_helix(&at, region, Vec3::ZERO, Vec3::Z, lead, (stop - start) / 360.0)
}

/// The rod cut by the screw sweeps of the section by y = 0 and the faces z = `z0`, `z1` of the bar
/// from (`x0`, `y0`, `z0`) to (`x1`, `y1`, `z1`), which points at the z axis across y = 0, carried
/// by the screw rising `lead` a turn from its turn of `start` degrees to that of `stop`.
pub fn bar_cut(rod: Body, a: (f64, f64, f64), b: (f64, f64, f64), lead: f64, start: f64, stop: f64) -> R<Body> {
    let ((x0, y0, z0), (x1, y1, z1)) = (a, b);
    let section = screw(&xz(), &rect(x0, z0, x1, z1), lead, start, stop)?;
    let level = |z| Plane { origin: v3(0.0, 0.0, z), x: Vec3::X, y: Vec3::Y };
    let mut cut_rod = cut(&rod, &section)?;
    for z in [z1, z0] {
        cut_rod = cut(&cut_rod, &screw(&level(z), &rect(x0, y0, x1, y1), lead, start, stop)?)?;
    }
    Ok(cut_rod)
}

/// What the closed `profile` passes through as the screw about the z axis that rises `lead` a
/// turn carries it from its turn of `start` degrees to that of `stop`, on solidcore-ops, whose
/// screw sweep holds its walls to the runner's `SCREW_TOLERANCE`: swept from where it is drawn,
/// then turned and raised to the screw's turn of `start`.
pub fn screw_ops(profile: &[Curve], lead: f64, start: f64, stop: f64) -> Result<OpsBody, Refusal> {
    let turns = (stop - start) / 360.0;
    let swept = screw_sweep(profile, [0.0; 3], [0.0, 0.0, 1.0], lead, turns, None, SCREW_TOLERANCE)?;
    let turned = rotated(swept, [0.0; 3], [0.0, 0.0, 1.0], start)?;
    translated(turned, [0.0, 0.0, lead * start / 360.0])
}

pub fn build() -> R<Body> {
    let rod = zcyl(0.0, 0.0, 5.0, 0.0, 20.0)?;
    bar_cut(rod, (4.0, -8.0, -1.0), (12.0, 8.0, 1.0), 6.0, -90.0, 1350.0)
}
