"""A ball united with a box at its center, the ball turned off the axes. A ball of radius 5 centered
on the origin, bounded by a plain spherical surface (not a B-spline): the sphere with its axis
along (1, 2, 2)/3 and its seam, the meridian where the surface's angle about the axis wraps, in
the half-plane through that axis and the direction (2, 1, −2)/3. So its poles are at ±(5/3, 10/3,
10/3), and neither the poles nor the seam lie in any of the planes x = 0, y = 0 or z = 0. Unite it
with the box from (0, 0, 0) to (8, 8, 8)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    frame = gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 2, 2), gp_Dir(2, 1, -2))
    ball = BRepPrimAPI_MakeSphere(frame, 5).Shape()
    box = BRepPrimAPI_MakeBox(8, 8, 8).Shape()
    fuse = BRepAlgoAPI_Fuse(ball, box)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the ball and the box")
    return fuse.Shape()
