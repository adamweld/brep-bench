"""A narrow cup's skin thickened inward by twice its bend's radius. A sheet of revolution about the
z axis, a narrow cup: a flat floor, the disk of radius 3 in the plane z = 0; a bend, the quarter
of a torus whose tube has radius 2 about the circle of radius 3 at z = 2, from the floor's edge
out and up to the circle of radius 5 at z = 2; and a wall, the cylinder of radius 5 from z = 2 up
to z = 10. Its faces meet tangentially. In the half-plane through the axis, with (ρ, z) the
distance from the axis and the height, it is the line from (0, 0) to (3, 0), the quarter circle
of radius 2 about (3, 2) to (5, 2), and the line up to (5, 10), turned a whole turn.

Thicken it by 4 to the inside, up from the floor and in from the wall. The thickened sheet is every
point p + s·n, for p on the sheet, n its unit normal on that side, and s from 0 to 4.

That is more than the bend's radius, 2. The bend's face, moved 4 toward its tube's center line,
would pass through it and turn inside out, a torus of tube radius −2; all of it lies inside the
solid and is no part of its boundary. The floor moved up 4 and the wall moved in 4 meet instead in a
concave edge, the circle of radius 1 at z = 4, and the bend fills the quarter disk behind it. In the
half-plane the solid's section is bounded by the sheet's own profile and the polyline from (5, 10)
through (1, 10), (1, 4) and (0, 4), closed down the axis."""
from math import cos, radians, sin

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Pnt

from thicken import thicken

THICKNESS = 4


def at(rho, z):
    return gp_Pnt(rho, 0, z)


def line(start, end):
    return BRepBuilderAPI_MakeEdge(at(*start), at(*end)).Edge()


def arc(center, radius, start, end):
    """The arc about `center` in the (ρ, z) half-plane from `start` to `end` degrees."""
    def on(angle):
        turn = radians(angle)
        return at(center[0] + radius * cos(turn), center[1] + radius * sin(turn))
    through = GC_MakeArcOfCircle(on(start), on((start + end) / 2), on(end)).Value()
    return BRepBuilderAPI_MakeEdge(through).Edge()


def sheet():
    """The floor, the bend and the wall: the profile turned about the z axis."""
    profile = BRepBuilderAPI_MakeWire()
    for edge in [
        line((0, 0), (3, 0)),
        arc((3, 2), 2, 270, 360),
        line((5, 2), (5, 10)),
    ]:
        profile.Add(edge)
    return BRepPrimAPI_MakeRevol(profile.Wire(), gp.OZ_s()).Shape()


def build():
    # the profile runs so that, turned about the axis, its faces' normals point out of the bend,
    # so inside is a negative distance
    return thicken(sheet(), -THICKNESS)
