//! The L-shaped plate on the outline (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), 8 thick,
//! with every face moved 1 outward and the moved faces run on until they meet, no round added:
//! springrep's face offset, which keeps the solid's topology.
use std::collections::HashMap;

use springrep::edit::offset_faces;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::offset::l_block;

pub fn build() -> Result<Brep, String> {
    let plate = l_block()?;
    let every_face: HashMap<_, _> = plate.face_ids().map(|f| (f, 1.)).collect();
    offset_faces(&plate, &every_face).map_err(e)
}
