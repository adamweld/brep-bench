"""A bull-nose mill pushed straight down into a block at a lean. A block, the box from (−8, −8, −8)
to (8, 8, 0), with a hole bored into its top by a bull-nose mill pushed straight down into it at a
lean, stopping wholly inside it.

The mill is a solid of revolution about an axis leaning 30° from upright toward +x, along the
direction a = (sin 30°, 0, cos 30°), through the point c = (0, 0, 4), the middle of its flat end.
Along its axis it runs from c up to c + 3a. It is a cylinder of radius 2 whose end corner is
rounded to radius 0.5: its flat end is a disc of radius 1.5 about c, square to a; its side, of
radius 2, runs from c + 0.5a to c + 3a, where it is flat; and the round between them is a quarter
of a torus whose tube, of radius 0.5, is centered on the circle of radius 1.5 about c + 0.5a.

It slides without turning straight down by 8, so that its end's middle ends at (0, 0, −4), the
whole mill under the block's top. Cut from the block everything the mill passes through."""
from bench import Refused


def build():
    # Pushed straight down, the leaning mill's side, a cylinder, touches along its two lines in
    # the plane through its axis square to the push's part across the axis, which a plane would
    # cut out, and its flat end leads all over. But its round is a quarter torus whose axis
    # leans to the push: at the point of its tube's circle at angle u about the axis and v round
    # the tube, the normal is square to the push where tan v depends on u, a curve that is none
    # of the torus's circles and no plane section of it. OCCT sweeps no solid, and a face it
    # slides sweeps to a solid only once split along its contact: finding that curve would be
    # computing the envelope outside the kernel.
    raise Refused("occt has no sweep of a solid: it slides faces, and the leaning mill's round, "
                  "a torus whose axis leans to the push, touches what it sweeps along a curve "
                  "that is none of its circles and no plane section of it, which occt has no "
                  "operation to find")
