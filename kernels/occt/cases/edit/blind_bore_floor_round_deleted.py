"""A blind bore's rounded floor deleted: the floor and the wall meet again. A block, the box from (−10, −10, 0) to (10, 10, 10), with a blind bore of radius 4 about the z axis
sunk from its top down to a flat floor at z = 4. The concave edge where the floor meets the bore's
wall is rounded to radius 1: the round is a quarter of a torus whose tube has radius 1 about the
circle of radius 3 at z = 5, tangent to the floor along the circle of radius 3 and to the wall
along the circle at z = 5.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The floor
runs on outward from radius 3 and the wall down from z = 5 until they meet in the circle of radius 4
at z = 4.

The result is the block with the sharp-floored bore."""
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
    block = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, 0), gp_Pnt(10, 10, 10)).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 4), gp_Dir(0, 0, 1)), 4, 7).Shape()
    solid = BRepAlgoAPI_Cut(block, bore).Shape()
    solid = rounded(solid, 1, edges_where(solid, on_circle((0, 0, 4), 4)), "the bore's floor edge")
    return deleted(solid, [nearest_face(solid, (0, -3.7071067811865475, 4.292893218813452))])
