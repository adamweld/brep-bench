"""A rounded box's rounded top rim deleted: the top and the walls meet again all round. A block, the box from the origin to (20, 12, 8), with its four upright edges rounded to radius 3, so its
outline is a rectangle with quarter circles of radius 3 at its corners. Then the whole rim of its
top, z = 8, is rounded to radius 1: four quarter cylinders along the straight sides, each about a
line 1 in from its side and 1 below the top, and four pieces of tori at the corners, each about a
quarter circle of radius 2 at z = 7 centered on the corner's upright axis. The eight run into one
another tangentially, a closed chain round the top.

Then edit it directly: delete the eight faces of the chain together. Deleting faces takes them out
of the solid and closes the gap they leave with the faces round them, each run on along its own
surface until they meet. The top runs on outward and the walls (the flat sides and the corner
cylinders) up until they meet in the top's rim again.

The result is the block with its upright edges rounded to radius 3 and its top rim sharp."""
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
    solid = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 12, 8)).Shape()
    solid = rounded(solid, 3, edges_where(solid, lambda m: abs(m.Z() - 4) < 1e-9), "the upright edges")
    solid = rounded(solid, 1, edges_where(solid, lambda m: abs(m.Z() - 8) < 1e-9), "the top rim")
    # Each face of the chain halfway round its tube.
    q = math.sqrt(0.5)
    s, a = q, 1 - q
    corner = lambda cx, cy, dx, dy: (cx + dx * (2 + s), cy + dy * (2 + s), 7 + s)
    points = [(10, a, 7 + s), (10, 12 - a, 7 + s), (a, 6, 7 + s), (20 - a, 6, 7 + s),
              corner(3, 3, -q, -q), corner(17, 3, q, -q), corner(17, 9, q, q), corner(3, 9, -q, q)]
    return deleted(solid, [nearest_face(solid, point) for point in points])
