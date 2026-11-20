"""A spherical cap's flat rim rounded with a radius that varies all the way round it. A cap of a ball: the part of the ball of radius 10 about the origin with z ≥ 6, a dome 4 high on a
flat circular face of radius 8 in the plane z = 6. Round the circular edge where the flat face meets
the sphere with a radius that varies all the way round it: at the angle φ about the z axis,
measured from +x toward +y, r = 5/4 + (cos φ)/4. So the radius is 1.5 at φ = 0, where the edge's
seam is, 1 opposite it at φ = 180°, and varies smoothly between.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is an arc of the circle of radius r(φ) that touches the flat face and the
sphere from inside. Its center is at (ρ, z) = (√(64 − 32r), 6 + r), r above the flat and 10 − r
from the ball's center. It touches the flat at (√(64 − 32r), 6) and the sphere at the point where
the ray from the ball's center through the circle's center reaches the sphere. The edge's angle
between the flat and the sphere is 53.13°."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused
from chamfer import edge_through
from vround import angle_about_z, rounded_with_law

RADIUS, FLAT = 10, 6


def build():
    ball = BRepPrimAPI_MakeSphere(RADIUS).Shape()
    below = BRepPrimAPI_MakeBox(gp_Pnt(-RADIUS, -RADIUS, -RADIUS), gp_Pnt(RADIUS, RADIUS, FLAT)).Shape()
    cut = BRepAlgoAPI_Cut(ball, below)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the cap from the ball")
    cap = cut.Shape()
    rim = edge_through(cap, (0, math.sqrt(RADIUS ** 2 - FLAT ** 2), FLAT))  # the circle at the flat
    return rounded_with_law(cap, rim, lambda p: 1.25 + 0.25 * math.cos(angle_about_z(p)),
                            "the cap's rim")
