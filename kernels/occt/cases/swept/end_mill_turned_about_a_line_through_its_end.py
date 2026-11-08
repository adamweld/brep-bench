"""A pocket cut by an end mill turned about a line through its flat end. A block, the box from
(−10, −10, −5) to (10, 10, 0), with a pocket cut into its top by an end mill turned about a line
that passes through its own flat end, as spot facing is done off center.

The mill is a solid cylinder of radius 3 standing upright about the vertical line through (1.5, 0),
from z = −2 up to z = 4: its flat end is 2 below the block's top, and the z axis passes through
that end, 1.5 from its center.

It turns rigidly about the z axis, right-handed (from +x toward +y), by 120°, so that its axis ends
on the vertical line through (1.5 cos 120°, 1.5 sin 120°). Cut from the block everything the mill
passes through.

The pocket's floor, z = −2, is the region the end's disc sweeps: the disc where it starts and where
it stops, and between them everything out to 4.5 from the axis. Near the axis the two discs and the
turn overlap, so the pocket's wall there is only the parts of the two discs' rims the other does
not cover."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.ShapeUpgrade import ShapeUpgrade_UnifySameDomain
from OCP.gp import gp, gp_Ax2, gp_Pnt

from sweep import cut, plane, turned

BLOCK = ((-10, -10, -5), (10, 10, 0))
FOOT = (1.5, 0, -2)  # the middle of the mill's flat end
RADIUS, HEIGHT = 3, 6
TURN = 120  # degrees about the z axis


def build():
    mill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*FOOT), gp.DZ_s()), RADIUS, HEIGHT).Shape()
    # The turn's axis and the mill's are parallel: its side meets the turn square-on along its
    # two lines in the plane y = 0 through both axes; its flat ends turn in their own planes.
    swept = turned(mill, gp.OZ_s(), TURN, [plane((0, 0, 0), (0, 1, 0))])
    pocketed = cut(BRepPrimAPI_MakeBox(gp_Pnt(*BLOCK[0]), gp_Pnt(*BLOCK[1])).Shape(), swept)
    # The pocket's floor is left in pieces, the mill's end where it starts and where it stops
    # and the flat sweep of its rim between them, overlapping about the axis; one of them
    # BRepMesh cannot mesh. Merged into the one face they make, it can.
    floor = ShapeUpgrade_UnifySameDomain(pocketed, True, True, True)
    floor.Build()
    return floor.Shape()
