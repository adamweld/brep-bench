"""A ring pushed straight down into a block at a lean. A block, the box from (−8, −8, −8) to (8, 8,
0), with a hole bored into its top by a ring pushed straight down into it at a lean, stopping
wholly inside it.

The ring is a torus centered at (0, 0, 4), its axis leaning 35° from upright toward +x, along the
direction (sin 35°, 0, cos 35°): the circle through its tube's centers has radius 3, and its tube
has radius 1. As placed it is wholly above the block.

It slides without turning straight down by 8, so that its center ends at (0, 0, −4), the whole ring
under the block's top. Cut from the block everything the ring passes through.

The ring touches what it sweeps along two closed curves, the outer and the inner, where its normal
is level. Each is carried down into an upright wall: the hole's, and the wall of the core the
ring's own hole leaves standing in the middle."""
from bench import Refused


def build():
    # Pushed straight down, the ring touches what it sweeps where its normal is level. Its axis
    # leans 35° to the push: at the point of its tube's circle at angle u about its axis and v
    # round the tube, that is where tan v depends on u, two closed curves that are none of the
    # torus's circles and no plane sections of it. OCCT sweeps no solid, and a face it slides
    # sweeps to a solid only once split along its contact: finding those curves would be
    # computing the envelope outside the kernel.
    raise Refused("occt has no sweep of a solid: it slides faces, and the leaning ring touches "
                  "what it sweeps along curves that are none of its circles and no plane "
                  "sections of it, which occt has no operation to find")
