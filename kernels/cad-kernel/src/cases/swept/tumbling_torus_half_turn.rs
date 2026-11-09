//! An oblique torus tumble through half a turn. See cases/swept/tumbling-torus-half-turn.md for the continuous solid.
use crate::tumble::Tumble;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Tumble {
        centre: [3., 0., 2.],
        hub: 2., tube: 0.16, ratio: 0.375,
        lean: 30., heading: 20., height: 0.7071,
        observer: 1., roll: [-90., 90.],
    }.build()
}
