//! The L-shaped block through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), 8 high, its
//! step, the riser x = 5 and the tread y = 5, then deleted together: the end y = 15 runs on to
//! x = 20 and the side x = 20 on to y = 15, meeting in a new edge along x = 20, y = 15, and the top
//! and the bottom close in over the corner.
//!
//! Refused. cad-kernel builds the L-block: `sketch::l_block`, its outline swept up by `prism`. What
//! is missing is the edit. cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`,
//! `rounds`, `offset` or `sew` takes faces out of a boundary and runs the faces round them on along
//! their own surfaces until they close the gap, and `offset::normal_sweep` sweeps a face along its
//! normal without extending any face beside it. Joining the missing corner back in would be the
//! result drawn again, not the step deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the riser x = 5 \
         and the tread y = 5 out and runs the end y = 15 on to x = 20 and the side x = 20 on to \
         y = 15 until they meet in a new edge, the top and the bottom closing over the corner"
        .into())
}
