"""A round post against a cone frustum's side, its wall drafted. A frustum about the z axis, radius
10 − z/5, z = 0 to 20, and a post of radius 4 about the vertical line through (11, 0), z = 5 to 15,
united. The post's wall is drafted 5°, pull +z, neutral plane z = 5, into a cone.

BRepOffsetAPI_DraftAngle on the fused solid returns a solid about 0.9% off in volume (a finding),
so this builds the drafted solid from exact pieces: the cone the wall becomes (radius 4 at z = 5,
4 − 10·tan 5° at z = 15), fused with the frustum."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    base = BRepPrimAPI_MakeCone(10, 6, 20).Shape()
    post = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(11, 0, 5), gp.DZ_s()),
                                4, 4 - 10 * math.tan(math.radians(5)), 10).Shape()
    fuse = BRepAlgoAPI_Fuse(base, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the post")
    return fuse.Shape()
