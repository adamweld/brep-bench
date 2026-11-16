"""A ball standing in a plate, its rounded root deleted: the plate and the ball meet again. A plate, the disk of radius 8 about the z axis from z = −2 to z = 0, and on it the ball of radius 3
about (0, 0, 1), joined to it: the ball stands up out of the plate's top through the circle of radius
√8 at z = 0. The concave edge where the plate's top meets the ball is rounded to radius 1: the round
is a piece of a torus whose tube has radius 1 about the circle of radius 4 at z = 1, tangent to the
top and to the ball.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The top
runs on inward and the ball down until they meet in the circle of radius √8 at z = 0.

The result is the plate and the ball joined, their edge sharp again."""
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
    plate = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -2), gp_Dir(0, 0, 1)), 8, 2).Shape()
    solid = BRepAlgoAPI_Fuse(plate, BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 1), 3).Shape()).Shape()
    solid = rounded(solid, 1, edges_where(solid, on_circle((0, 0, 0), math.sqrt(8))), "the ball's root")
    return deleted(solid, [nearest_face(solid, (0, -3.2393960073426595, 0.3507838831685456))])
