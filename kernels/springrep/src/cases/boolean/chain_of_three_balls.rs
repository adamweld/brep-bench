//! Balls of radius 5, 4 and 3 about the origin, (7, 0, 0) and (10, 4, 0), joined one after another:
//! the middle one overlaps both others, which do not touch.
use springrep::topo::Brep;

use crate::shared::boolean::chain_of_three_balls;

pub fn build() -> Result<Brep, String> {
    chain_of_three_balls()
}
