"""A drilled block's two rounded hole rims deleted: the faces and the bore meet again at both ends. A block, the box from the origin to (20, 10, 10), drilled through from top to bottom by a hole of
radius 2 about the vertical line x = 10, y = 5, with both of the hole's rims rounded to radius 1:
at the top a quarter of a torus whose tube has radius 1 about the circle of radius 3 at z = 9, and
at the bottom its mirror image about the circle of radius 3 at z = 1, each centered on the hole's
axis and tangent to its face and to the hole's wall.

Then edit it directly: delete both rounds together. Deleting faces takes them out of the solid and
closes the gaps they leave with the faces round them, each run on along its own surface until they
meet. The top and the bottom run on inward from radius 3, and the wall up from z = 9 and down from z
= 1, until they meet in the circles of radius 2 at z = 10 and at z = 0.

The result is the block drilled through by the hole of radius 2, both its rims sharp."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeVertex
from OCP.BRepExtrema import BRepExtrema_DistShapeShape
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.TopAbs import TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from blend import edges_where, rounded
from edit import deleted, on_circle


def nearest_face(solid, point):
    """The face of `solid` nearest `point` (OCCT's rounds are close to the exact tori but not on
    them to the nanometre, so a point on the torus picks its face by distance)."""
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
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 10, 10)).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(10, 5, -1), gp_Dir(0, 0, 1)), 2, 12).Shape()
    solid = BRepAlgoAPI_Cut(block, bore).Shape()
    solid = rounded(solid, 1, edges_where(solid, on_circle((10, 5, 10), 2)) + edges_where(solid, on_circle((10, 5, 0), 2)),
                    "the hole's rims")
    # Each round at 45° round its tube, about the circle of radius 3 at z = 9 or 1.
    s = math.sqrt(2) / 2
    return deleted(solid, [nearest_face(solid, (10, 5 - (3 - s), 9 + s)),
                           nearest_face(solid, (10, 5 - (3 - s), 1 - s))])
