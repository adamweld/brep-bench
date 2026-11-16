"""A half torus's rounded rims deleted: the flat and the torus meet again inside and out. The half of the ring torus about the z axis whose tube has radius 2 about the circle of radius 6 that lies above
the plane z = 0: its flat face the ring 4 ≤ r ≤ 8 at z = 0. Both its rims rounded to radius 1/2: the outer, the circle of radius 8 at z = 0, by a piece of a torus whose tube has radius 1/2 about the circle of radius 6 + √2 at z = 1/2, and the inner, the circle of radius 4, by one about the circle of radius 6 − √2 at z = 1/2, each tangent to the flat and to the half torus.

Then edit it directly: delete both rounds together. Deleting a face takes it out of the solid and closes the gap it leaves
with the faces round it, each run on along its own surface until they meet. The flat runs on inward and outward and the torus
down until they meet in the circles of radius 4 and 8 at z = 0.

The result is the half torus."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeVertex
from OCP.BRepExtrema import BRepExtrema_DistShapeShape
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeTorus
from OCP.TopAbs import TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from blend import edges_where, rounded
from edit import deleted, on_circle


def nearest_face(solid, point):
    """The face of `solid` nearest `point` (OCCT's rounds on a torus are B-splines, close to the exact tori but not on
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
    above = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, 0), gp_Pnt(10, 10, 5)).Shape()
    half = BRepAlgoAPI_Common(BRepPrimAPI_MakeTorus(6, 2).Shape(), above).Shape()
    solid = rounded(half, 0.5, edges_where(half, on_circle((0, 0, 0), 8)) + edges_where(half, on_circle((0, 0, 0), 4)), "the rims")
    # Each round halfway round its tube, toward the old rim.
    return deleted(solid, [nearest_face(solid, point) for point in [(0, -7.794515558701765, 0.1753919415842729), (0, -4.205484441298235, 0.1753919415842729)]])
