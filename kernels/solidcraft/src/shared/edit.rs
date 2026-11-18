//! What the implementations of the edit cases share.
use crate::bench::{cut, edges_through_each, faces_through_each, order, reloaded};
use crate::solids::{KernelError, Vec3, R};
use crate::Refusal;
use solidcore::blend::fillet_removal::delete_fillet_face;
use solidcore::topology::brep::is_outward_solid;
use solidcore::{FaceId, Kernel};
use solidcore_convert::brep::{to_kernel, to_record};
use solidcore_ops::{blend, make_box, make_cylinder, offset_faces, Blend, Body, Corners};

/// The box from the origin to `size`, drilled right through along z by a hole of `radius` on the
/// vertical line through (`x`, `y`): a cylinder running past both faces, cut from the box.
pub fn drilled_box(size: [f64; 3], x: f64, y: f64, radius: f64) -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], size)?;
    let drill = make_cylinder([x, y, -1.0], radius, size[2] + 2.0)?;
    cut(block, drill)
}

/// The faces of `body` through `points` moved `distance` along their outward normals (negative:
/// inward), the faces around them following: ops' face offset, solidcore's direct edit.
pub fn move_faces(body: &Body, points: &[[f64; 3]], distance: f64) -> Result<(), Refusal> {
    let faces = faces_through_each(body, points)?;
    offset_faces(body, &order(body)?, &faces, distance)?;
    Ok(())
}

/// `body` with the edges through `points` rounded to `radius`, an edge named twice taken once:
/// ops' blend, on the body read back from its record, whose edges ops' blend numbers as the
/// record does ([`reloaded`]).
pub fn rounded(body: &Body, points: &[[f64; 3]], radius: f64) -> Result<Body, Refusal> {
    let (body, order) = reloaded(body)?;
    let mut edges = edges_through_each(&body, &order, points)?;
    edges.sort_unstable();
    edges.dedup();
    blend(&body, Blend::Round, &edges, &[radius], &[], Corners::Filled)?;
    Ok(body)
}

/// The faces of SolidCraft's `body` nearest each of `points`, a face named twice taken once, by
/// their index in its face order: what the kernel's `delete_faces` takes. A face is named by a
/// point on it, a round or a bevel by a point of the sharp edge it replaced.
pub fn faces_nearest(body: &solidcraft_kernel::Body, points: &[Vec3]) -> R<Vec<usize>> {
    let mut faces = Vec::new();
    for p in points {
        let (_, f) = body
            .nearest_face(*p)
            .ok_or_else(|| KernelError::Invalid(format!("no face near {:?}", [p.x, p.y, p.z])))?;
        if !faces.contains(&f) {
            faces.push(f);
        }
    }
    Ok(faces)
}

/// The record faces of `body` nearest each of `points`, a face named twice taken once: a round
/// named by a point of the sharp edge it replaced.
pub fn rounds_nearest(body: &Body, points: &[[f64; 3]]) -> Result<Vec<usize>, Refusal> {
    let rows = body.face_distances_within(&order(body)?, points, 1e3)?;
    let mut faces = Vec::new();
    for (i, p) in points.iter().enumerate() {
        let near = rows
            .iter()
            .enumerate()
            .filter_map(|(f, row)| Some((row.get(i).copied().flatten()?, f)))
            .min_by(|a, b| a.0.total_cmp(&b.0));
        let (_, f) = near.ok_or_else(|| format!("no face near {p:?}"))?;
        if !faces.contains(&f) {
            faces.push(f);
        }
    }
    Ok(faces)
}

/// Eight points round the circle of `radius` about the z axis at height `z`, clear of the
/// quarter turns: a whole rim named, however many faces it is split into.
pub fn round_the_rim(radius: f64, z: f64) -> Vec<[f64; 3]> {
    (0..8)
        .map(|k| {
            let a = 0.1 + k as f64 * std::f64::consts::FRAC_PI_4;
            [radius * a.cos(), radius * a.sin(), z]
        })
        .collect()
}

/// The rounds of `body` nearest `points` ([`rounds_nearest`]) deleted, one face after another,
/// by the kernel's fillet removal (`delete_fillet_face`, SMLib's `SmBrep::DeleteFilletFace`): the
/// round taken out and the gap closed by its two tangent faces, each run on along its own
/// surface to the sharp edge where they meet. A round all the way round, with no ends, goes to
/// the fork's ring removal (the fix `fillet_removal_takes_ring_fillets`, on with the others).
///
/// Neither SolidCraft's kernel nor ops offers this: the kernel's `delete_faces` heals with planar
/// neighbours only, and ops' `remove_faces` leaves an open sheet. So the kernel does it itself,
/// on the body's record made its own, with the fixes ops's session runs with and editing enabled
/// as the removal requires, and the result is held to be a closed solid facing out.
pub fn delete_rounds(body: &Body, points: &[[f64; 3]]) -> Result<Body, Refusal> {
    let rounds = rounds_nearest(body, points)?;
    let (record, _) = body.to_record()?;
    let mut k = Kernel::default();
    for name in solidcore_ops::session::fixes() {
        if let Some(on) = k.fixes.flag_mut(name) {
            *on = true;
        }
    }
    let (mut brep, order) =
        to_kernel(&mut k, &record, true).map_err(|e| format!("the record does not convert: {e:?}"))?;
    // the kernel's faces, named before the first removal changes anything
    let faces: Vec<FaceId> = rounds.iter().map(|&i| order.faces[i]).collect();
    let editing = brep.editing_enabled();
    brep.set_editing_enabled(true);
    for f in faces {
        delete_fillet_face(&mut k, &mut brep, f, None)?;
    }
    brep.set_editing_enabled(editing);
    if !is_outward_solid(&mut k, &mut brep)? {
        return Err("the result is not a closed solid".into());
    }
    let (record, _) =
        to_record(&mut k, &mut brep).map_err(|e| format!("the result makes no record: {e:?}"))?;
    Ok(Body::from_record(&record, true)?.0)
}
