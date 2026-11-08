"""A ring pushed into a block at so steep a lean its inner outline folds. A block, the box from
(−8, −8, −8) to (8, 8, 0), with a hole bored into its top by a ring pushed straight down into it at
a lean, stopping wholly inside it.

The ring is a torus centered at (0, 0, 4), its axis leaning 65° from upright toward +x, along the
direction (sin 65°, 0, cos 65°): the circle through its tube's centers has radius 3, and its tube
has radius 1. As placed it is wholly above the block.

It slides without turning straight down by 8, so that its center ends at (0, 0, −4), the whole ring
under the block's top. Cut from the block everything the ring passes through.

Leaning past tan² β = (R − r)/r (here 54.7°), the curve along which the ring's normal is level,
seen from above, cusps: the inner outline folds back on itself, and the wall it sweeps down is cut
where it folds."""
from bench import Refused


def build():
    # Pushed straight down, the ring touches what it sweeps where its normal is level. Its axis
    # leans 65° to the push: at the point of its tube's circle at angle u about its axis and v
    # round the tube, that is where tan v depends on u, two closed curves that are none of the
    # torus's circles and no plane sections of it, the inner one cusped seen from above. OCCT
    # sweeps no solid, and a face it slides sweeps to a solid only once split along its
    # contact: finding those curves would be computing the envelope outside the kernel.
    raise Refused("occt has no sweep of a solid: it slides faces, and the leaning ring touches "
                  "what it sweeps along curves that are none of its circles and no plane "
                  "sections of it, which occt has no operation to find")
