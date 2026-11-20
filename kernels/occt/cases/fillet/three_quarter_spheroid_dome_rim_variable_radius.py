"""Three quarters of a spheroid dome, its base rim rounded with a radius growing along it. The quarter
of the ellipse (ρ/10)² + (z/5)² ≤ 1, ρ, z ≥ 0, turned about the z axis from φ = 0° to 270°. Round the
three-quarter circle where the base meets the spheroid with r = 1 + 2φ/(3π), φ the angle about the z
axis from +x toward +y: 1 at the end face φ = 0°, 2 at φ = 270°. The round's sections in the planes
through the axis are the circle of radius r(φ) inside the dome touching the base and the ellipse, and
the round ends in the two flat end faces."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from chamfer import edge_through
from vround import angle_about_z, rounded_with_law

TURN = 3 * math.pi / 2


def build():
    # The quarter ellipse in the plane y = 0: its axes along x (10) and z (5), run from (10, 0, 0) to
    # (0, 0, 5), closed by the two radii along the axes.
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0)), 10, 5)
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(ellipse, 0, math.pi / 2).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 5), gp_Pnt(0, 0, 0)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 0), gp_Pnt(10, 0, 0)).Edge())
    dome = BRepPrimAPI_MakeRevol(BRepBuilderAPI_MakeFace(wire.Wire()).Face(), gp.OZ_s(), TURN).Shape()
    s = 10 * math.sqrt(0.5)
    rim = edge_through(dome, (s, s, 0))  # the three-quarter circle round the base

    # r = 1 + 2φ/(3π), φ from 0 to 3π/2; atan2's angle below 0 is the one past π.
    def radius(p):
        phi = angle_about_z(p)
        return 1 + 2 * (phi + 2 * math.pi if phi < -1e-6 else phi) / (3 * math.pi)

    return rounded_with_law(dome, rim, radius, "the dome's base rim", stations=64)
