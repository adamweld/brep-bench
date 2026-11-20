"""A post united with a larger ball centred on its top, its wall cutting the ball's surface. A post: a solid cylinder of radius 3 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A ball of radius 5 centred on the middle of the post's top face, (0, 0, 10). Make the union of the post and the ball. The ball is wider than the post: it stands 5 above the top face, and its lower part, down to z = 5, surrounds the post's top. The ball's surface cuts the post's wall in the circle ρ = 3, z = 6. Above that circle, the post's wall and top are inside the ball; below it the wall runs on to the base, at z = 0, clear of the ball."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 3, 10).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 10), 5).Shape()
    fuse = BRepAlgoAPI_Fuse(post, ball)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
