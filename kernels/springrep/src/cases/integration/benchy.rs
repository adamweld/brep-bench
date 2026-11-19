//! #3DBenchy, the tugboat from its designer's CAD file.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep makes no face from a B-spline control net and sews no faces into a solid in \
         its public API (its free-form faces come only from a slab's height field, a loft's skin \
         or a sweep), so the hull's surface, a bicubic net, cannot be built"
        .into())
}
