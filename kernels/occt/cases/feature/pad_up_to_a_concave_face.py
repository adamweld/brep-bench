"""A square pad extruded out to the inside of a half ring. A half ring, and a square pad that runs
out from near its axis until it meets the ring's inside.

The half ring: the rectangle 20 ≤ x ≤ 26, 0 ≤ z ≤ 10 in the plane y = 0, revolved half a turn about
the z axis from +x toward +y. So it is everything between 20 and 26 from the z axis with y ≥ 0 and 0
≤ z ≤ 10, its two flat ends in the plane y = 0.

The pad: the square |x| ≤ 4, 2 ≤ z ≤ 8 in the plane y = 0 (in the ring's opening, clear of its
ends), extruded along +y up to the first face it meets, the ring's inner side, the cylinder of
radius 20 about the z axis, and joined to the ring.

So the pad's four flat sides run from y = 0 out to that cylinder, which they reach at y = √(20² −
x²): 20 at the square's middle, x = 0, and √384 = 8√6 ≈ 19.596 at its sides x = ±4. The pad's far
end is the patch of the cylinder in the square's path, which it shares with the ring. The pad and
the ring are one solid."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFeat import BRepFeat_MakePrism
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.TopoDS import TopoDS_Face
from OCP.gp import gp, gp_Dir, gp_Pnt

from bench import Refused, face_through

RING = [(20, 0), (26, 0), (26, 10), (20, 10)]  # (x, z) in the plane y = 0
SQUARE = [(-4, 2), (4, 2), (4, 8), (-4, 8)]


def flat(corners):
    outline = BRepBuilderAPI_MakePolygon()
    for x, z in corners:
        outline.Add(gp_Pnt(x, 0, z))
    outline.Close()
    return BRepBuilderAPI_MakeFace(outline.Wire()).Face()


def build():
    ring = BRepPrimAPI_MakeRevol(flat(RING), gp.OZ_s(), math.pi).Shape()
    inside = face_through(ring, (0, 20, 5))  # the ring's inner side, radius 20
    # A boss (fuse) from the square, which lies on no face of the ring, along +y to its inside.
    pad = BRepFeat_MakePrism(ring, flat(SQUARE), TopoDS_Face(), gp_Dir(0, 1, 0), 1, True)
    pad.Perform(inside)
    if not pad.IsDone():
        raise Refused(f"BRepFeat_MakePrism could not extrude the pad: {pad.CurrentStatusError()}")
    return pad.Shape()
