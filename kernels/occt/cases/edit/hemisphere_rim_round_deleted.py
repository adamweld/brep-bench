"""A hemisphere's rounded rim deleted: the flat and the ball meet again. The half of the ball of radius 5 about the origin above the plane z = 0, with its rim, where the flat
meets the ball, rounded to radius 1: the round is a piece of a torus whose tube has radius 1 about
the circle of radius √15 at z = 1, tangent to the flat and to the ball.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The flat
runs on outward and the ball down until they meet in the circle of radius 5 at z = 0.

The result is the hemisphere again."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeVertex
from OCP.BRepExtrema import BRepExtrema_DistShapeShape
from OCP.BRepPrimAPI import (BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder,
                             BRepPrimAPI_MakeSphere)
from OCP.TopAbs import TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from blend import edges_where, rounded
from edit import deleted, on_circle


def nearest_face(solid, point):
    """The face of `solid` nearest `point` (OCCT's rounds on balls are B-splines, close to the
    torus but not on it to the nanometre, so a point on the torus picks its face by distance)."""
    vertex = BRepBuilderAPI_MakeVertex(gp_Pnt(*point)).Vertex()
    best = None
    faces = TopExp_Explorer(solid, TopAbs_FACE)
    while faces.More():
        face = TopoDS.Face(faces.Current())
        gap = BRepExtrema_DistShapeShape(vertex, face)
        gap.Perform()
        if best is None or gap.Value() < best[0]:
            best = (gap.Value(), face)
        faces.Next()
    return best[1]


def build():
    below = BRepPrimAPI_MakeBox(gp_Pnt(-6, -6, -6), gp_Pnt(6, 6, 0)).Shape()
    solid = BRepAlgoAPI_Cut(BRepPrimAPI_MakeSphere(5).Shape(), below).Shape()
    solid = rounded(solid, 1, edges_where(solid, on_circle((0, 0, 0), 5)), "the hemisphere's rim")
    return deleted(solid, [nearest_face(solid, (0, -4.620983133994804, 0.3363010339995828))])
