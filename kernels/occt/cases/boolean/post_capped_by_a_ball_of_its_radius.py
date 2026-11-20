"""A post capped by a ball of its own radius centred on its top. A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A ball of radius 5 centred on the middle of the post's top face, (0, 0, 10). Make the union of the post and the ball. The ball's equator is the post's top rim; the ball's surface meets the post's wall tangentially all round that circle, so the two run on smoothly into one another there. Below z = 10 the ball is inside the post and goes. What is left is the post with a hemispherical dome."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 10), 5).Shape()
    fuse = BRepAlgoAPI_Fuse(post, ball)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
