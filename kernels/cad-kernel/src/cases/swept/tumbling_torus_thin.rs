//! A thin torus tumbled through space. See cases/swept/tumbling-torus-thin.md for the continuous solid.
use crate::tumble::Tumble;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Tumble {
        centre: [3., 0., 2.],
        hub: 2., tube: 0.015, ratio: 0.25,
        lean: 45., heading: 0., height: 0.5_f64.sqrt(),
        observer: 1., roll: [-75., 75.],
    }.build()
}
