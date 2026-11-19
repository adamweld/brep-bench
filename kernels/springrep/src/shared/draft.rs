//! Helpers shared by the draft cases: the faces to draft, picked by the names their features gave
//! them, and the neutral plane a draft tilts them about.
use springrep::edit;
use springrep::math::Frame;
use springrep::pick::faces_named;
use springrep::topo::{Brep, FaceId};

use crate::shared::{at, e};

/// Every face each of `names` stands for (those carrying it, and every face on its surface); an
/// error naming a name that stands for none.
pub fn faces(b: &Brep, names: &[&str]) -> Result<Vec<FaceId>, String> {
    let mut out = Vec::new();
    for &name in names {
        let found = faces_named(b, name);
        if found.is_empty() {
            return Err(format!("no face is named {name}"));
        }
        out.extend(
            found
                .into_iter()
                .filter(|f| !out.contains(f))
                .collect::<Vec<_>>(),
        );
    }
    Ok(out)
}

/// Only the faces carrying `name` itself, not the others on their surface.
pub fn carrying(b: &Brep, name: &str) -> Result<Vec<FaceId>, String> {
    let out: Vec<FaceId> = b
        .face_ids()
        .filter(|&f| b.face(f).names.iter().any(|n| n.to_string() == name))
        .collect();
    if out.is_empty() {
        return Err(format!("no face is named {name}"));
    }
    Ok(out)
}

/// The neutral plane z = `z`, its normal the pull direction +z.
pub fn neutral(z: f64) -> Frame {
    at([0., 0., z])
}

/// `b` with the faces `names` stand for drafted by `degrees` about the plane z = `z`, pulled
/// along +z: positive angles lean them inward as they rise.
pub fn draft_up(b: &Brep, names: &[&str], z: f64, degrees: f64) -> Result<Brep, String> {
    let picked = faces(b, names)?;
    edit::draft(b, &picked, &neutral(z), degrees.to_radians())
        .map_err(|err| format!("springrep's draft declines: {}", e(err)))
}
