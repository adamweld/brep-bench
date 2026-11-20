"""A post on a block under a parabolic dome, its side drafted. The solid turned about the z axis
from the region under the quadratic Bézier arc from (ρ, z) = (0, 10) through (10, 10) to (20, 4),
z = 10 − 0.015ρ², above z = 0, united with the cylinder of radius 6 about the z axis from z = 0 to
z = 30; the cylinder's side drafted by 4°, pull +z, neutral plane z = 0, into the cone
ρ = 6 − z·tan 4°."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp

from bench import face_through
from draft import drafted, revolved_bezier_block, united


def build():
    block = revolved_bezier_block([(0, 10), (10, 10), (20, 4)], 20)
    post = BRepPrimAPI_MakeCylinder(gp.XOY_s(), 6, 30).Shape()
    part = united(block, post)
    side = face_through(part, (6, 0, 20))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
