//! The box from the origin to (20, 12, 8), its four upright edges rounded to radius 3, then the
//! whole rim of its top rounded to radius 1, four quarter cylinders and four pieces of tori running
//! into one another in a closed chain; the eight faces of the chain then deleted together, the top
//! running on outward and the walls up until they meet in the top's rim again.
//!
//! Refused. cad-kernel builds the rounded box: its outline, a `sketch::rounded_rectangle`, swept up
//! by `prism`, the top's rim rolled as one chain by `fillet::roll` and the round's piece cut away.
//! What is missing is the edit. cad-kernel deletes no face of a solid: nothing in `boolean`,
//! `fillet`, `rounds`, `offset` or `sew` takes faces out of a boundary and runs the faces round
//! them on along their own surfaces until they close the gap, and `offset::normal_sweep` sweeps a
//! face along its normal without extending any face beside it. Joining back the ring the chain took
//! would be the result drawn again, not the chain deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the eight faces of \
         the top's rounded rim out and runs the top on outward and the flat sides and the corner \
         cylinders up until they meet in the top's rim at z = 8"
        .into())
}
