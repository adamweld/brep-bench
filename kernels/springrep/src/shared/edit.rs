//! Helpers shared by the edit cases: faces picked by the names their features gave them, a face
//! deletion by those names, and the parts several cases start from.
use springrep::edit::delete_faces;
use springrep::math::{Point3, Transform, Vec3};
use springrep::pick::faces_named;
use springrep::primitive::cylinder;
use springrep::topo::{Brep, FaceId};

use crate::shared::{at, e};

/// Every face each of `names` stands for (those carrying it, and every face on its surface); an
/// error naming a name that stands for none.
pub fn faces(b: &Brep, names: &[&str]) -> Result<Vec<FaceId>, String> {
    let mut out: Vec<FaceId> = Vec::new();
    for &name in names {
        let found = faces_named(b, name);
        if found.is_empty() {
            return Err(format!("no face is named {name}"));
        }
        for f in found {
            if !out.contains(&f) {
                out.push(f);
            }
        }
    }
    Ok(out)
}

/// Every face a feature named, whatever it called it: all of a blend's faces, say.
pub fn of_feature(b: &Brep, feature: &str) -> Result<Vec<FaceId>, String> {
    let out: Vec<FaceId> = b
        .face_ids()
        .filter(|&f| b.face(f).names.iter().any(|n| n.feature == feature))
        .collect();
    if out.is_empty() {
        return Err(format!("no face was made by {feature}"));
    }
    Ok(out)
}

/// `b` with `faces` taken out and the gap they leave closed by the faces round them.
pub fn delete(b: &Brep, faces: &[FaceId]) -> Result<Brep, String> {
    delete_faces(b, faces).map_err(|err| format!("springrep's delete_faces declines: {}", e(err)))
}

/// `b` with the faces `names` stand for deleted.
pub fn delete_named(b: &Brep, names: &[&str]) -> Result<Brep, String> {
    delete(b, &faces(b, names)?)
}

/// `b` with every face `feature` made deleted.
pub fn delete_feature(b: &Brep, feature: &str) -> Result<Brep, String> {
    delete(b, &of_feature(b, feature)?)
}

/// `b` turned by `radians` about the z axis.
pub fn turned(mut b: Brep, radians: f64) -> Brep {
    b.transform(&Transform::rotation(Point3::ZERO, Vec3::Z, radians));
    b
}

/// The shaft several cases start from: the cylinder of radius 5 about the z axis from z = 0 to
/// z = 20; its faces `shaft.start`, `shaft.end` and `shaft.side`.
pub fn shaft() -> Result<Brep, String> {
    cylinder(&at([0., 0., 0.]), 5., 20., "shaft").map_err(e)
}
