"""A rod threaded by a cone's point. A rod, the cylinder of radius 5 about the z axis from z = 0 to
z = 20, threaded by a pointed cone that points at the rod's axis, carried along a screw, as an
engraving bit chases a thread.

The cone's axis is the x axis and its point is (4, 0, 0), 1 inside the rod. It opens away from the
rod at 60° (each side 30° from its axis) to a flat base at x = 12, of radius 8 tan 30°.

The screw is about the z axis: the cone turns right-handed about it (from +x toward +y) and rises 6
for every whole turn, rigidly. It runs from a turn of −90° (lowered 1.5, so that all of it within 5
of the z axis is below the rod) to a turn of 1350° (raised 22.5, all of that above it). Cut from
the rod everything the cone passes through.

The cone's point lies on its axis, and the groove's root is the helix it runs along: the contact on
each side runs down a root to the point and back up the other."""
from bench import Refused


def build():
    # The cone's axis meets the screw's, but the screw also rises. At the point of its side at
    # distance s from the point along its axis and angle t round it, the normal is square to
    # the velocity ẑ × p + kẑ where tan t = −A(s)/(k cos 30°), A linear in s: a curve that
    # crosses the cone's rulings, no plane section and none of its circles. OCCT sweeps no
    # solid, and a face it carries along the screw (BRepOffsetAPI_MakePipeShell) sweeps to a
    # solid only once split along its contact: finding that curve would be computing the
    # envelope outside the kernel.
    raise Refused("occt has no sweep of a solid: it carries faces along a screw, and the cone "
                  "touches what it sweeps along a curve crossing its rulings, which the screw's "
                  "rise makes no plane section of it, and which occt has no operation to find")
