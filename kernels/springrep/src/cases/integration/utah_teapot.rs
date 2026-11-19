//! The Utah teapot: Newell's 32 bicubic Bézier patches made into four solids and united.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep makes no face from a Bézier patch's control points and sews no faces into a \
         solid in its public API: its free-form faces come only from a slab's height field over a \
         rectangle, a loft's skin or a sweep, and none of those is a teapot's patch"
        .into())
}
