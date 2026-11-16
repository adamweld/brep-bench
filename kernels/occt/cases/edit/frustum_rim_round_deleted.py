"""A cone frustum's rounded top rim deleted: the top and the cone meet again. A cone frustum on the z axis, its base the disk of radius 6 at z = 0 and its top the disk of radius 3
at z = 6, with its top rim rounded to radius 1: the round is a piece of a torus whose tube has radius
1 about the circle of radius (7 − √5)/2 at z = 5, tangent to the top and to the cone.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The top
runs on outward and the cone up until they meet in the circle of radius 3 at z = 6.

The result is the frustum from radius 6 at z = 0 to radius 3 at z = 6."""
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
    solid = BRepPrimAPI_MakeCone(6, 3, 6).Shape()
    solid = rounded(solid, 1, edges_where(solid, on_circle((0, 0, 6), 3)), "the frustum's rim")
    return deleted(solid, [nearest_face(solid, (0, -2.9076971233692386, 5.850650808352039))])
