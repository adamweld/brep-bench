"""A ball and a cube turned about an edge that touches it, their common part. The ball of radius 5
about the origin, its poles on the z axis and its seam in the half-plane y = 0, x > 0, and the cube
of side 10 whose corner is (5, 0, −5) and whose edges from it run along (4/5, 3/5, 0),
(−3/5, 4/5, 0) and the z axis: the cube from (5, 0, −5) to (15, 10, 5) turned by asin(3/5) about
its edge on the line x = 5, y = 0, which touches the ball at (5, 0, 0)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    # OCCT's sphere has its poles on z and its seam in the half-plane y = 0, x > 0
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 5).Shape()
    # the box's x runs along the placement's x direction, its y along z × x = (−3/5, 4/5, 0)
    cube = BRepPrimAPI_MakeBox(gp_Ax2(gp_Pnt(5, 0, -5), gp.DZ_s(), gp_Dir(4, 3, 0)),
                               10, 10, 10).Shape()
    common = BRepAlgoAPI_Common(ball, cube)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube")
    return common.Shape()
