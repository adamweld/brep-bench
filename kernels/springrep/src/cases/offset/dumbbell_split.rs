//! The dumbbell plate, two 10 × 10 squares joined by a neck 4 long and 2 wide, 6 thick, shrunk by
//! 1.5: the neck vanishes and both halves are kept, two lumps of one solid.
//! springrep 1.0.1 builds it wrong: the complement's offset, as the slot of the neck closes, loses
//! the rounds about the neck's mouth (each body comes out a plain block, without its bulge).
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::offset::shrunk;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let outline = [
        (0., 0.), (10., 0.), (10., 4.), (14., 4.), (14., 0.), (24., 0.),
        (24., 10.), (14., 10.), (14., 6.), (10., 6.), (10., 10.), (0., 10.),
    ]
    .map(|(x, y)| Vec2::new(x, y));
    let plate = extrude(&Profile::polygon(&outline, &["wall"; 12]), &at([0., 0., 0.]), 6., "plate")
        .map_err(e)?;
    shrunk(&plate, 1.5)
}
