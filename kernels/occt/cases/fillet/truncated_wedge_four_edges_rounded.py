"""A truncated wedge with its four edges along its depth rounded, each to its own radius. A truncated wedge: the prism on the quadrilateral through (0, 0), (20, 0), (12, 10) and (0, 10) in
the xy plane, from z = 0 to z = 8. It has a flat bottom 20 wide, a flat top 12 wide, an upright face
x = 0, a slanting face from (20, 0) to (12, 10), and two flat ends z = 0 and z = 8.

Round the four edges along z, the ones through the quadrilateral's four corners, each to a radius of
its own: 1 at (0, 0), 2 at (20, 0), 3 at (12, 10) and 1.5 at (0, 10). Each round is a cylinder, its
axis along z, tangent to the two faces of its edge, and it runs the whole depth, from one flat end to
the other. No two of these edges share an end, so no round meets another: the quadrilateral's corners
are rounded in the plane and the result is that quadrilateral extruded. The corners' angles are 90°
at (0, 0) and (0, 10), 51.34° at (20, 0) and 128.66° at (12, 10)."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeWedge
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet

from blend import edges_where
from bench import Refused

TOLERANCE = 1e-7
# The quadrilateral's corners, and the radius at each.
RADII = {(0, 0): 1, (20, 0): 2, (12, 10): 3, (0, 10): 1.5}


def build():
    # A wedge 20 by 10 by 8 whose top face is 12 long: the right trapezoid, extruded along z.
    wedge = BRepPrimAPI_MakeWedge(20, 10, 8, 12).Shape()
    fillet = BRepFilletAPI_MakeFillet(wedge)
    for (x, y), radius in RADII.items():
        [edge] = edges_where(wedge, lambda p: abs(p.X() - x) < TOLERANCE and abs(p.Y() - y) < TOLERANCE)
        fillet.Add(radius, edge)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the wedge's four edges")
    return fillet.Shape()
