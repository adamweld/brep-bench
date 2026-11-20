"""A widening cone post on a dome, its side drafted to lean the other way. A dome, the half of the
ball of radius 20 centered on the origin with z ≥ 0, and a post, the cone about the z axis from
z = 0 to z = 30 whose radius 6 + z·tan 2° grows as it rises, united.

Draft the post's side by 4°, with the pull direction +z and the neutral plane z = 0: the side
becomes the cone through the same circle of radius 6 at z = 0, leaning 4° from the z direction,
inward as it rises, its radius 6 − z·tan 4°."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeSphere
from OCP.gp import gp

from bench import face_through
from draft import drafted, united


def build():
    dome = BRepPrimAPI_MakeSphere(20, 0, math.pi / 2).Shape()
    post = BRepPrimAPI_MakeCone(gp.XOY_s(), 6, 6 + 30 * math.tan(math.radians(2)), 30).Shape()
    part = united(dome, post)
    side = face_through(part, (6 + 25 * math.tan(math.radians(2)), 0, 25))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
