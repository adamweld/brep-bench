"""A triangular ring segment, one of its cones moved out. A ring segment of triangular section: the triangle with corners (4, 0, 0), (10, 0, 0) and (7, 0, 4),
in the half-plane y = 0, x ≥ 0, turned about the z axis through a third of a turn (120°), from
φ = 0° to φ = 120°, where φ is the angle about the z axis from +x toward +y. Its faces are a flat
bottom in the plane z = 0, a sector of the annulus between radii 4 and 10; two conical faces, the
inner one swept by the side from (4, 0, 0) to (7, 0, 4) and the outer one swept by the side from
(10, 0, 0) to (7, 0, 4); and two flat triangles, in φ = 0° and φ = 120°.

Then edit the solid directly: move the inner cone 2 outward along its outward normal, which points
up and in. The cone moves to the cone coaxial with it and parallel to it, 2 away: the cone through
the circle of radius 3/2 in the plane z = 0 and (23/4, 17/3) in a half-plane through the axis. The
faces beside it follow: the bottom runs on in to radius 3/2, the outer cone is cut to meet the
moved cone, at the height 17/3 and radius 23/4, and the two flat triangles are the section of the
new solid by their half-planes, the triangle with corners (3/2, 0), (10, 0) and (23/4, 17/3) in
radius and height.

So the result is the ring segment of triangular section whose triangle has those corners, the same
third of a turn."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt

from bench import face_through
from edit import moved


def ring():
    """The triangle (4, 0), (10, 0), (7, 4) in radius and height, in the half-plane y = 0, turned
    about the z axis through 120 degrees."""
    polygon = BRepBuilderAPI_MakePolygon()
    for x, z in [(4, 0), (10, 0), (7, 4)]:
        polygon.Add(gp_Pnt(x, 0, z))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire(), True).Face()
    return BRepPrimAPI_MakeRevol(face, gp.OZ_s(), 2 * math.pi / 3).Shape()


def build():
    solid = ring()
    a = math.radians(60)  # the inner cone, at its middle: radius 5.5, height 2
    cone = face_through(solid, (5.5 * math.cos(a), 5.5 * math.sin(a), 2))
    return moved(solid, [(cone, 2)])
