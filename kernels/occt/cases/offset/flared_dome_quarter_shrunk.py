"""A flared dome turned a quarter of a turn, shrunk by 1. The solid turned about the z axis, through
angles φ from 0° to 90° (φ measured from +x toward +y), from the region of the (ρ, z) half-plane
bounded below by the line z = 0 from ρ = 0 to ρ = 10, on the outside by the straight line from (ρ,
z) = (10, 0) to (6, 4) and then by the quarter of the ellipse (ρ/6)² + ((z − 4)/8)² = 1 from (6,
4) up to (0, 12), and on the inside by the axis. So it is a flat base disk of radius 10, a cone
flaring in to radius 6 at height 4, and a dome, a quarter of an ellipsoid of semi-axes 6, 6 and 8,
rising from there to a pole at (0, 0, 12). The cone and the dome meet at a crease, the circle ρ =
6, z = 4, where the surface turns inward and then upward: a concave edge. Its other faces are two
flat cut faces, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the half-plane x = 0, y ≥ 0 (φ =
90°), which meet along the z axis, from z = 0 to z = 12, in a convex edge."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips

from offset import offset
from turned import at, line, turned

TURN = math.pi / 2


def build():
    # the ellipse about (0, 0, 4), its long axis along z (8) and its short one along x (6)
    ellipse = gp_Elips(gp_Ax2(at(0, 4), gp_Dir(0, 1, 0), gp_Dir(0, 0, 1)), 8, 6)
    dome = BRepBuilderAPI_MakeEdge(ellipse, 0, math.pi / 2).Edge()  # from (0, 12) to (6, 4)
    flared = turned([line(at(0, 0), at(10, 0)), line(at(10, 0), at(6, 4)), dome,
                     line(at(0, 12), at(0, 0))], TURN, "the flared dome")
    return offset(flared, -1)
