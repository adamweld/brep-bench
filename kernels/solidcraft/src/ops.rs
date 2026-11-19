//! The cases built on solidcore-ops, the modelling layer under SolidCraft's kernel crate, where
//! solidcraft-kernel has no operation for what a case describes (or a worse one): an ops body
//! written as STEP and, where the writer declines it, measured and meshed with solidcore's own
//! tools.
//!
//! These implementations came from brep-bench's solidcore runner, which tested the same engine
//! from beneath SolidCraft; they keep its conventions (`crate::bench`, `crate::shared`).
use crate::Refusal;
use serde_json::{Value, json};
use solidcore_ops::{Body, CheckLevel};
use solidcore_record::brep::Surface;
use solidcore_tess::{MeshTolerance, plan};
use std::collections::BTreeMap;

impl From<solidcore_ops::Error> for Refusal {
    fn from(e: solidcore_ops::Error) -> Self {
        Refusal(e.to_string())
    }
}

impl From<solidcore::SolidError> for Refusal {
    fn from(e: solidcore::SolidError) -> Self {
        Refusal(e.to_string())
    }
}

/// Every fix solidcore makes to the SMLib it was ported from, on for this process. They are off
/// by default, so that solidcore's default path matches SMLib; the ops cases were written with
/// them all on, as solidcore's own runner ran them. SolidCraft's kernel picks its own fixes for
/// the SMLib calls it makes (`solidcraft_smlib`), so this is done only before an ops case. The
/// fixes not taking is the runner's failure, not the kernel declining: a crash, not a refusal.
pub fn set_fixes() -> Result<(), Refusal> {
    solidcore_ops::session::set_fixes(solidcore::kernel::UpstreamFixes::NAMES.iter().copied())
        .unwrap_or_else(|e| panic!("the fixes are not set: {e:?}"));
    Ok(())
}

/// A face's surface as the reply counts it.
fn kind(s: &Surface) -> &'static str {
    match s {
        Surface::Plane { .. } | Surface::PlaneThrough { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        // a round is recorded as a cone of no angle: a cylinder
        Surface::Cone { half_angle, .. } if *half_angle == 0.0 => "cylinder",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } | Surface::BallPatch { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        Surface::Spline { .. } | Surface::RationalSpline { .. } => "bspline",
        _ => "other",
    }
}

/// The triangles written as binary STL.
fn write_stl(path: &str, mesh: &solidcore_tess::Mesh) -> std::io::Result<()> {
    let mut out = Vec::with_capacity(84 + 50 * mesh.triangles.len());
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&(mesh.triangles.len() as u32).to_le_bytes());
    for t in &mesh.triangles {
        let [a, b, c] = t.map(|i| mesh.positions[i as usize]);
        let (u, v) = ([b.x - a.x, b.y - a.y, b.z - a.z], [c.x - a.x, c.y - a.y, c.z - a.z]);
        let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        let n = if l > 0.0 { n.map(|x| x / l) } else { n };
        for x in [n[0], n[1], n[2], a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z] {
            out.extend_from_slice(&(x as f32).to_le_bytes());
        }
        out.extend_from_slice(&[0u8; 2]);
    }
    std::fs::write(path, out)
}

/// The relative accuracies the body is integrated at, finest first: the kernel's finest, 1e-8,
/// does not always converge (a ball's volume), and each measure is taken at the finest that does.
const ACCURACIES: [f64; 5] = [1e-8, 3e-8, 1e-7, 3e-7, 1e-6];

/// A measure at the finest of [`ACCURACIES`] the kernel converges at, or the last error.
fn finest(measure: impl Fn(f64) -> solidcore_ops::Result<f64>) -> solidcore_ops::Result<f64> {
    let mut last = Err(solidcore_ops::Error::Argument);
    for a in ACCURACIES {
        last = measure(a);
        if last.is_ok() {
            break;
        }
    }
    last
}

/// The body written as STEP (AP242) into `path` by solidcore-step, the writer SolidCraft's own
/// export goes through (`solidcraft_kernel::step_export` writes a body's record with it), or why
/// not: the writer's words where it declines.
pub fn step(body: &Body, path: &str) -> Option<String> {
    let header = solidcore_step::write::product::Header {
        originating_system: "SolidCraft".into(),
        ..Default::default()
    };
    match solidcore_step::write::product::solidcore_bodies(&[("Body1", body)], &header) {
        Ok(text) => {
            std::fs::write(path, text).expect("the STEP file writes");
            None
        }
        Err(e) => Some(e.to_string()),
    }
}

/// The reply for an ops body the STEP writer declined, or with no STEP asked for: its volume and
/// area integrated over the exact B-rep, its faces by surface kind as its record names them and
/// the kernel's validity check, its triangles (solidcore-tess) written to `out`. The mesh keeps
/// within a ten-thousandth of the body's size, or 1/200 of its thickness (its volume over its
/// area) where that is less.
pub fn report(body: &Body, out: &str) -> Result<Value, Refusal> {
    let volume = finest(|a| body.volume(a)).map_err(|e| format!("volume not measured: {e}"))?;
    let area = finest(|a| body.area(a)).map_err(|e| format!("area not measured: {e}"))?;
    let valid = body.check(CheckLevel::Geometry).is_ok_and(|r| r.is_valid());
    let (record, _) = body.to_record().map_err(|e| format!("no record: {e}"))?;
    let mut faces: BTreeMap<&str, usize> = BTreeMap::new();
    for f in &record.faces {
        *faces.entry(kind(&f.surface)).or_default() += 1;
    }
    let (lo, hi) = body.bounds().map_err(|e| format!("no bounds: {e}"))?;
    let size = ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt();
    let thickness = if area > 0.0 && volume > 0.0 { volume / area } else { size };
    let chord = (1e-4 * size).min(5e-3 * thickness).max(1e-7);
    let (tess, stats) =
        plan(&record, &MeshTolerance::chord(chord), size).map_err(|e| format!("not meshed: {e}"))?;
    if let Some(f) = stats.unmeshed.first() {
        return Err(format!("not meshed: face {}: {}", f.face, f.reason).into());
    }
    let (mesh, left_out) = tess.evaluate();
    if let Some(f) = left_out.first() {
        return Err(format!("not meshed: face {}: {}", f.face, f.reason).into());
    }
    write_stl(out, &mesh).map_err(|e| format!("{out} not written: {e}"))?;
    Ok(json!({"status": "ok", "volume": volume, "area": area, "faces": faces, "valid": valid}))
}
