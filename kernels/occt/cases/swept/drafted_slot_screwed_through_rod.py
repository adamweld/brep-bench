"""A rod threaded by a drafted bar of slotted section. A rod, the cylinder of radius 5 about the z
axis from z = 0 to z = 20, threaded by a drafted bar whose section is a slot, carried along a
screw.

The bar points at the rod's axis. Its section in the plane x = 12 is the slot (stadium) made of the
two half discs of radius 1 centered at (12, ±1.5, 0) and the rectangle |y| ≤ 1.5, |z| ≤ 1 between
them: 5 long along y and 2 wide along z. The bar is that section carried along −x to x = 4, drafted
3° inward: in the plane x = 12 − s its section is the slot's shrunk by s·tan 3° on every side (its
half discs of radius 1 − s tan 3° about the same centers). So its sides are two planes and two half
cones, and its tip, the plane x = 4, is 1 inside the rod.

The screw is about the z axis: the tool turns right-handed about it (from +x toward +y) and rises 6
for every whole turn, rigidly. It runs from a turn of −90° (lowered 1.5, the tool wholly below the
rod) to a turn of 1350° (raised 22.5, wholly above it). Cut from the rod everything the bar passes
through."""
from bench import Refused


def build():
    # The bar's flat sides touch the screw along lines a plane would cut out. But its two half
    # cones have axes parallel to the x axis 1.5 either side of it, which run past the screw's
    # axis without meeting it: along each, the normal is square to the screw's velocity on a
    # curve that crosses the cone's rulings, no plane section and none of its circles. OCCT
    # sweeps no solid, and a face it carries along the screw sweeps to a solid only once split
    # along its contact: finding that curve would be computing the envelope outside the
    # kernel.
    raise Refused("occt has no sweep of a solid: it carries faces along a screw, and the slot's "
                  "half cones, whose axes run past the screw's, touch what they sweep along "
                  "curves that are no plane sections of them, which occt has no operation to "
                  "find")
