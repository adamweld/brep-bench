"""A round post against a cone frustum's side, its wall drafted. A frustum about the z axis, radius
10 − z/5, z = 0 to 20, and a post of radius 4 about the vertical line through (11, 0), z = 5 to 15,
united. The post's wall is drafted 5°, pull +z, neutral plane z = 5, into a cone."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused, face_through
from draft import drafted


def build():
    base = BRepPrimAPI_MakeCone(10, 6, 20).Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(11, 0, 5), gp.DZ_s()), 4, 10).Shape()
    fuse = BRepAlgoAPI_Fuse(base, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the post")
    part = fuse.Shape()
    return drafted(part, [face_through(part, (11, 4, 10))], 5, neutral=(0, 0, 5))
