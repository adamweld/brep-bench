"""A ball of radius 5 about the origin, and a cylinder of radius 2 lying along the x axis direction, its axis the line through (0, 0, 3) parallel to the x axis, from x = −8 to x = 8. Make their union.

The cylinder's axis passes 3 above the ball's centre, which is the ball's radius less the cylinder's, so the cylinder's topmost line, z = 5, just touches the ball's surface at its pole (0, 0, 5), and only there. The cylinder's wall otherwise crosses the ball's surface in a curve of two loops, one on each side of the plane x = 0, that meet at the pole.
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 5).Shape()
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-8, 0, 3), gp.DX_s()), 2, 16).Shape()
    result = BRepAlgoAPI_Fuse(ball, cylinder)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the ball and the cylinder")
    return result.Shape()
