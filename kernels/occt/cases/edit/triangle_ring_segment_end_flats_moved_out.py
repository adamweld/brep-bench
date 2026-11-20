"""A triangular ring segment, both its end flats moved out. A ring segment of triangular section: the triangle with corners (4, 0, 0), (10, 0, 0) and (7, 0, 4),
in the half-plane y = 0, x ≥ 0, turned about the z axis through a third of a turn (120°), from
φ = 0° to φ = 120°, where φ is the angle about the z axis from +x toward +y. Its faces are a flat
bottom in the plane z = 0, a sector of the annulus between radii 4 and 10; two conical faces, the
inner one swept by the side from (4, 0, 0) to (7, 0, 4) and the outer one swept by the side from
(10, 0, 0) to (7, 0, 4); and two flat triangles, in φ = 0° and φ = 120°, whose outward normals are
−y and (−√3/2, −1/2, 0), the directions of decreasing and of increasing φ.

Then edit the solid directly: move each of the two flat triangles 2 outward along its outward
normal. The first, in φ = 0°, goes to the plane y = −2; the second, in φ = 120°, to the
plane −(√3/2)x − y/2 = 2. The bottom and the
two cones run on to meet them, so each of them now spans, at distance ρ from the axis, the angle
from −asin(2/ρ) to 120° + asin(2/ρ), and each moved flat is bounded by two hyperbola-like curves
where the planes meet the cones and the bottom; the two planes meet at a line inside the hole
(distance 4/√3 from the axis), clear of the solid."""
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
    first = face_through(solid, (7, 0, 1))  # the triangle in y = 0
    b = math.radians(120)
    second = face_through(solid, (7 * math.cos(b), 7 * math.sin(b), 1))  # the one at phi = 120
    return moved(solid, [(first, 2), (second, 2)])
