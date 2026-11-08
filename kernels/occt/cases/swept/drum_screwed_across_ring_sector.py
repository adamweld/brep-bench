"""A helical groove milled by a drum screwed across a block. A curved block with a helical groove
milled through it by a drum screwed across it, as an end mill mills a helical groove.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The drum is a solid cylinder of radius 1 about the vertical line through (10, 0), from z = −0.5 to
z = 0.5, its ends flat.

It is carried along a screw about the z axis: it turns right-handed about it (from +x toward +y)
and rises 0.5 for every whole turn, rigidly, from a turn of −140° to one of +140°. It starts clear
of the block, passes wholly through it, and ends clear of it. Cut from the block everything the
drum passes through.

The drum's flat ends keep no plane: each sweeps a thin helical ramp."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from sweep import cut, plane, ring_sector, screw, screwed

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
AXIS = (10, 0)  # the drum stands on the vertical line through this point
RADIUS, LOW, HIGH = 1, -0.5, 0.5
LEAD = 0.5  # rise per whole turn
START, STOP = -140, 140  # degrees


def build():
    drum = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*AXIS, LOW), gp.DZ_s()), RADIUS, HIGH - LOW)
    tool = screw(drum.Shape(), LEAD, START)
    # The drum's side, upright, has level normals, square to the screw's rise: it meets the
    # screw square-on where it meets the turn, along its lines in the plane through both axes.
    # Its flat ends rise or sink all over.
    through_axes = screw(plane((0, 0, 0), (0, 1, 0)), LEAD, START)
    return cut(ring_sector(*BLOCK), screwed(tool, LEAD, START, STOP, [through_axes]))
