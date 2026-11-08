"""A helical groove milled by a drum leaning to the screw's axis. A curved block with a helical
groove milled through it by a drum leaning to the screw that carries it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The drum is a solid cylinder of radius 0.8 and length 3, its ends flat, whose axis runs through
(10, 0, 0), its middle, along the direction (0, sin 25°, cos 25°): leaning 25° from upright toward
+y. It stands out above and below the block.

It is carried along a screw about the z axis: it turns right-handed about it (from +x toward +y)
and rises 1 for every whole turn, rigidly, from a turn of −140° to one of +140°. It starts clear of
the block, passes wholly through it, and ends clear of it. Cut from the block everything the drum
passes through."""
from bench import Refused


def build():
    # The drum's axis leans across the screw's and runs past it without meeting it. At the
    # point of its side at height s along its axis and angle t round it, the normal is square
    # to the screw's velocity where a relation in s and t holds that is no plane section, no
    # line and no circle of the cylinder. OCCT sweeps no solid, and a face it carries along the
    # screw sweeps to a solid only once split along its contact: finding that curve would be
    # computing the envelope outside the kernel.
    raise Refused("occt has no sweep of a solid: it carries faces along a screw, and the "
                  "leaning drum's side touches what it sweeps along curves that are no plane "
                  "sections of it, which occt has no operation to find")
