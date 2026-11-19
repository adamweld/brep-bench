//! A plate, the box (0, 0, −20)–(100, 80, 0), less what a ball-end mill of radius 3 sweeps in 41
//! zigzag raster passes over the surface z = −5 + 4 sin(πx/50) sin(πy/40), 944 aslant moves.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. Built from its other operations, each aslant move's \
         sweep is a cylinder along the move with a ball at each end for the mill's ball, and \
         vertical cylinders, lofted oblique cylinders and a sheared box for its shank; but the \
         balls alone, cut from the plate along the first pass (each ball tangent to the cylinders \
         either side of it), already fail in boolean::difference_all with \"numerical failure: \
         face split: a hole lies in no region\", and the program has 944 such moves"
        .into())
}
