"""A flared dome turned three quarters of a turn, shrunk by 1. The solid turned about the z axis,
through angles φ from 0° to 270° (φ measured from +x toward +y), from the region of the (ρ, z) half-
plane bounded below by the line z = 0 from ρ = 0 to ρ = 10, on the outside by the straight line from
(ρ, z) = (10, 0) to (6, 4) and then by the quarter of the ellipse (ρ/6)² + ((z − 4)/8)² = 1 from (6,
4) up to (0, 12), and on the inside by the axis. So it is a flat base disk of radius 10, a cone
flaring in to radius 6 at height 4, and a dome, a quarter of an ellipsoid of semi-axes 6, 6 and 8,
rising from there to a pole at (0, 0, 12), all three quarters of a turn. The cone and the dome meet
at a crease, the circle ρ = 6, z = 4. Its other faces are two flat cut faces, in the half-plane y =
0, x ≥ 0 (φ = 0°) and in the half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the z axis, from z
= 0 to z = 12, in a concave edge.

Shrink it by 1: the solid of every point at least 1 from its outside. The base moves up to z = 1,
the cone moves in along its normals, the dome moves in as the surface of every point 1 inside the
ellipsoid, and the two cut faces move in 1, to the planes y = 1 (for x ≥ 0) and x = −1 (for y ≤ 0).
Where they would meet, round the edge of the missing quarter, the result is rounded by the quarter
of a cylinder of radius 1 about the z axis, the surface of every point 1 from the axis. The offset
of an ellipse has no closed form, so the case is unsettled until two kernels agree."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from bench import Refused
from offset import offset


def build():
    def at(rho, z):
        return gp_Pnt(rho, 0, z)

    # the ellipse about (0, 0, 4), its long axis along z (8) and its short one along x (6)
    ellipse = gp_Elips(gp_Ax2(at(0, 4), gp_Dir(0, 1, 0), gp_Dir(0, 0, 1)), 8, 6)
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(at(0, 0), at(10, 0)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(at(10, 0), at(6, 4)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(ellipse, 0, math.pi / 2).Edge())  # from (0, 12) to (6, 4)
    wire.Add(BRepBuilderAPI_MakeEdge(at(0, 12), at(0, 0)).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True)
    if not face.IsDone():
        raise Refused("BRepBuilderAPI_MakeFace made no face of the profile")
    turned = BRepPrimAPI_MakeRevol(face.Face(), gp.OZ_s(), 3 * math.pi / 2)
    if not turned.IsDone():
        raise Refused("BRepPrimAPI_MakeRevol declined the profile")
    return offset(turned.Shape(), -1)
