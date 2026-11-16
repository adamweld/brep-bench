"""A cone frustum's two rounded rims deleted: its ends and its cone meet again. A cone frustum on the z axis, from the disk of radius 6 at z = 0 to the disk of radius 3 at z = 6 (its side the cone
2r + z = 12), with both its rims rounded to radius 3/2: each round a piece of a torus of tube radius 3/2 tangent to the
end and to the cone.

Then edit it directly: delete both rounds together. Deleting a face takes it out of the solid and closes the gap it leaves with the
faces round it, each run on along its own surface until they meet. The top and the base run on outward and the cone up and down until they meet in the circles of radius 3 at z = 6 and radius 6 at z = 0.

The result is the frustum from radius 6 at z = 0 to radius 3 at z = 6."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeVertex
from OCP.BRepExtrema import BRepExtrema_DistShapeShape
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.TopAbs import TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from blend import edges_where, rounded
from edit import deleted, on_circle


def nearest_face(solid, point):
    """The face of `solid` nearest `point` (OCCT's rounds on a cone are close to the exact tori but not on them to the
    nanometre, so a point on the torus picks its face by distance)."""
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
    rims = edges_where(solid, on_circle((0, 0, 6), 3)) + edges_where(solid, on_circle((0, 0, 0), 6))
    solid = rounded(solid, 1.5, rims, "the rims")
    # Each round deleted, halfway round its tube toward its old rim.
    return deleted(solid, [nearest_face(solid, point) for point in [(0, -2.861545685053858, 5.77597621252806), (0, -4.848925229403217, 0.7114033318212998)]])
