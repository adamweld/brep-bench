"""A ball on a leaning stem swung partway into a block. A curved block with a notch cut into it by
a ball on a leaning stem, swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The tool is the union of a ball of radius 0.8 centered at (10, 0, 0) and a stem, a solid cylinder
of radius 0.4 whose axis runs from the ball's center 3 along the direction (0, sin 20°, cos 20°),
leaning 20° from upright toward +y, its far end flat. The stem stands out through the block's top.
Where the stem leaves the ball the tool has a concave seam.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the tool passes
through."""
from bench import Refused


def build():
    # A turn about the z axis meets the ball square-on along its great circle in the plane
    # through the axis and its centre, which a plane would cut out. But the stem's axis runs
    # past the turn's without meeting it: at the point of the stem's side at height s along its
    # axis and angle t round it, the normal is square to the velocity where a relation in s and
    # t holds that is no plane section, no line and no circle of the cylinder. OCCT sweeps no
    # solid (BRepPrimAPI_MakeRevol: "Solids are not Processed"), and a face it turns sweeps to a
    # solid only once split along its contact, which for the stem is that curve: finding it
    # would be computing the envelope outside the kernel.
    raise Refused("occt has no sweep of a solid: it turns faces, and the stem's side, whose "
                  "axis runs past the turn's without meeting it, touches what it sweeps along a "
                  "curve that is no section of it by a plane, which occt has no operation to "
                  "find")
