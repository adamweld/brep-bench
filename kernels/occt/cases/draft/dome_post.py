"""A post on a dome, its side drafted from the dome's base. A dome, the half of the ball of radius
20 centered on the origin with z ≥ 0, and a post, the cylinder of radius 5 about the z axis from
z = 0 to z = 30, united. The post's side shows only above the sphere, from z = √375 ≈ 19.365 to
its top at z = 30.

Draft the post's side by 3°, with the pull direction +z and the neutral plane z = 0, the dome's
flat base: the side becomes the cone through the post's circle of radius 5 at z = 0, leaning 3°
from the z direction, inward as it rises, its radius 5 − z·tan 3°. It still runs down into the
dome, now meeting the sphere where (5 − z·tan 3°)² + z² = 400, at z₀ ≈ 19.601, and the top
shrinks to meet it, to radius 5 − 30·tan 3° ≈ 3.428. The dome and the post's top are not drafted.

So the result is the dome with a frustum of the cone standing on it."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp

from bench import Refused, face_through
from draft import drafted


def build():
    dome = BRepPrimAPI_MakeSphere(20, 0, math.pi / 2).Shape()
    post = BRepPrimAPI_MakeCylinder(gp.XOY_s(), 5, 30).Shape()
    fuse = BRepAlgoAPI_Fuse(dome, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the post")
    part = fuse.Shape()
    side = face_through(part, (5, 0, 25))
    return drafted(part, [side], 3, neutral=(0, 0, 0))
