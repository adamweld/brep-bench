//! What the solidcore implementations of the edit cases that delete faces with curved
//! neighbours share: the deletion itself.
//!
//! SolidCraft's `delete_faces` heals a gap only by extending planar neighbours, and
//! solidcore-ops's `remove_faces` leaves the gap open, so the deletion is the kernel's own
//! defeature (`SmFeatureExecutive`), run on the body's record made its own ([`in_kernel`]): each
//! connected set of the faces is taken off as a feature, and the hole it leaves in the rest is
//! patched from the faces round it (`SM_RB_EXTEND_SURFACE`). Where the hole is an inner loop of
//! a face round it, the patch is that face's own surface inside the loop; else it is the face's
//! surface extended past its sides (`Surface::extended_sides`), the extensions trimmed to the
//! hole's edges. Those extensions are the neighbours' own surfaces run on, not surfaces fitted:
//! a plane is widened as itself, and a cone (a cylinder among them), sphere or torus has its own
//! net continued, each new point built on the analytic, so it runs on as itself, written as a
//! B-spline (solidcore `surface/extend/analytic.rs`).
use crate::bench::faces_through_each;
use crate::shared::offset::in_kernel;
use crate::Refusal;
use solidcore::blend::feature_executive::{FeatureExecutive, RebuildBrepType};
use solidcore::topology::{edge, face};
use solidcore::{Brep, FaceId};
use solidcore_ops::Body;

/// `body` with the faces through `points` deleted together, the gap they leave closed by the
/// faces round them run on along their surfaces.
pub fn delete_faces(body: &Body, points: &[[f64; 3]]) -> Result<Body, Refusal> {
    let picked = faces_through_each(body, points)?;
    let mut kept = None;
    let healed = in_kernel(body, |k, b, order| {
        let faces: Vec<FaceId> = picked.iter().filter_map(|&i| order.faces.get(i).copied()).collect();
        let groups = connected(b, &faces)?;
        {
            let mut ex = FeatureExecutive::new(k, b)?;
            for group in &groups {
                // the base's patch is the extended surfaces; the feature taken off (discarded) is
                // closed from the base's patch, as solidcore's own face offset calls it
                ex.add_feature(k, group, RebuildBrepType::ExtendSurface, RebuildBrepType::FromOtherBrep)?;
            }
            // a feature whose patch could not be built is put back, the others taken off
            let back = ex.do_defeature(k)?;
            if !back.is_empty() {
                kept = Some(format!(
                    "the kernel's defeature took off {} of the {} sets of faces and put the rest back: {:?}",
                    groups.len() - back.len(),
                    groups.len(),
                    back.iter().map(|f| f.limit).collect::<Vec<_>>()
                ));
            }
        }
        b.refresh_cached_topology();
        Ok(std::mem::take(b))
    })?;
    match kept {
        Some(why) => Err(why.into()),
        None => Ok(healed),
    }
}

/// `faces` of `b` in sets that share edges: each set one feature.
fn connected(b: &Brep, faces: &[FaceId]) -> solidcore::Result<Vec<Vec<FaceId>>> {
    let mut groups: Vec<Vec<FaceId>> = Vec::new();
    for &f in faces {
        let mut touching = Vec::new();
        for e in face::p4::edges(b, f, None, false, None)? {
            for g in edge::faces(b, e, None)? {
                touching.push(g);
            }
        }
        // the sets f touches, merged with it into one
        let mut merged = vec![f];
        groups.retain(|set| {
            let touches = set.iter().any(|g| touching.contains(g));
            if touches {
                merged.extend(set);
            }
            !touches
        });
        groups.push(merged);
    }
    Ok(groups)
}
