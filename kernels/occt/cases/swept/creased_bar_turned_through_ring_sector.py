"""A groove cut by a bar drafted both ways from a parting line. A curved block grooved by a bar
drafted both ways from the plane its section is drawn in, as a tool forged or cast from a parting
line is, turned past the block.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The bar is drawn in the plane through the origin spanned by u = (1, 0, 0) and v = (0, cos 20°, −sin
20°), leaning 20° across the turn; its normal is n = (0, sin 20°, cos 20°). The bar is every point
a u + b v + h n with |h| ≤ 4, 10 + |h| tan 2° ≤ a ≤ 13 − |h| tan 2° and |b| ≤ 2 − |h| tan 2°: the
rectangle 10 ≤ a ≤ 13, |b| ≤ 2 in that plane, its four sides drafted 2° inward both ways from it.
Each side folds at the plane in a convex crease.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +140°: it starts
clear of the block, passes through it, and ends clear of it. Cut from the block everything the bar
passes through."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopoDS import TopoDS
from OCP.gp import gp, gp_Pnt

from bench import Refused
from sweep import cut, flat_contacts_turned, found, ring_sector, turn, turned

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
TILT = math.radians(20)
DRAFT = math.tan(math.radians(2))
INNER, OUTER, HALF_WIDTH = 10, 13, 2  # the rectangle in the parting plane, in (a, b)
HALF_HEIGHT = 4  # |h|
START, STOP = -140, 140  # degrees about the z axis


def point(a, b, h):
    u, v, n = (1, 0, 0), (0, math.cos(TILT), -math.sin(TILT)), (0, math.sin(TILT), math.cos(TILT))
    return gp_Pnt(*(a * x + b * y + h * z for x, y, z in zip(u, v, n)))


def bar():
    """The bar's ten flat faces sewn into a solid: its rectangle in the parting plane shrunk by
    |h| tan 2° on every side at h = ±4, its ends there, and the four sides between, each folded
    at the parting plane."""
    def corners(h):
        shrink = abs(h) * DRAFT
        return [point(a, b, h) for a, b in ((INNER + shrink, -HALF_WIDTH + shrink),
                                              (OUTER - shrink, -HALF_WIDTH + shrink),
                                              (OUTER - shrink, HALF_WIDTH - shrink),
                                              (INNER + shrink, HALF_WIDTH - shrink))]
    low, middle, high = corners(-HALF_HEIGHT), corners(0), corners(HALF_HEIGHT)
    loops = [low, high]
    for i in range(4):
        j = (i + 1) % 4
        loops += [[low[i], low[j], middle[j], middle[i]], [middle[i], middle[j], high[j], high[i]]]
    sewing = BRepBuilderAPI_Sewing(1e-7)
    for loop in loops:
        sewing.Add(BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakePolygon(*loop, True).Wire()).Face())
    sewing.Perform()
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(found(sewing.SewedShape(), TopAbs_SHELL)[0]))
    if not solid.IsDone():
        raise Refused("BRepBuilderAPI_MakeSolid declined the bar's sewn faces")
    solid = solid.Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid


def build():
    tool = turn(bar(), gp.OZ_s(), START)
    # Each flat face meets the turn square-on along the plane through the axis and its normal,
    # which cuts it along its contact where it reaches it.
    swept = turned(tool, gp.OZ_s(), STOP - START, flat_contacts_turned(tool, gp.OZ_s()))
    return cut(ring_sector(*BLOCK), swept)
