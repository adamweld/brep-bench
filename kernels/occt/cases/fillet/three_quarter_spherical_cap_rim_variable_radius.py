"""Three quarters of a spherical cap, its flat rim rounded with a radius growing along it. The part
of the ball of radius 10 about the origin with z ≥ 6, turned from φ = 0° to 270° about the z axis.
Round the three-quarter circle where the flat face meets the sphere, with r = 1 + φ/(3π), φ the angle
about the z axis from +x toward +y: 1 at the end face φ = 0°, 3/2 at φ = 270°. The round's sections
in the planes through the axis are the circle of radius r(φ) touching the flat and the sphere from
inside, and the round ends in the two flat end faces."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from chamfer import edge_through
from vround import angle_about_z, rounded_with_law

RADIUS, FLAT = 10, 6
TURN = 3 * math.pi / 2


def build():
    rho_flat = math.sqrt(RADIUS ** 2 - FLAT ** 2)  # 8
    # The cap's section in the plane y = 0 (local x = +x, local y = +z): the flat from the axis to
    # (8, 0, 6), the arc of the great circle up to (0, 0, 10), and the axis back down.
    circle = gp_Circ(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0)), RADIUS)
    arc = BRepBuilderAPI_MakeEdge(circle, math.asin(FLAT / RADIUS), math.pi / 2).Edge()
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, FLAT), gp_Pnt(rho_flat, 0, FLAT)).Edge())
    wire.Add(arc)
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, RADIUS), gp_Pnt(0, 0, FLAT)).Edge())
    cap = BRepPrimAPI_MakeRevol(BRepBuilderAPI_MakeFace(wire.Wire()).Face(), gp.OZ_s(), TURN).Shape()
    rim = edge_through(cap, (rho_flat * math.sqrt(0.5), rho_flat * math.sqrt(0.5), FLAT))
    # r = 1 + φ/(3π), φ from 0 to 3π/2; atan2's angle below 0 is the one past π.
    def radius(p):
        phi = angle_about_z(p)
        return 1 + (phi + 2 * math.pi if phi < -1e-6 else phi) / (3 * math.pi)

    return rounded_with_law(cap, rim, radius, "the cap's flat rim", stations=64)
