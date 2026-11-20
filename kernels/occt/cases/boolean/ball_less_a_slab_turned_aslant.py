"""A ball of radius 5 about the origin, less a thick slab turned aslant to the axes. The slab is the flat box with its two big faces in the planes n \cdot p = -1 and n \cdot p = 2, where n = (1, 2, 2)/3, so it is 3 thick and its faces lie 1 and 2 from the ball's centre, on opposite sides of it. It is a box 30 by 30 in those planes, centred on the point 0.5\,n, with sides along u = (2, -2, 1)/3 and v = (2, 1, -2)/3, which with n make a right-handed frame. It reaches well past the ball in every direction across its faces, so it cuts the ball in two.

What is left is two caps of the ball, a small one 3 high beyond the plane n \cdot p = 2 and a bigger one 4 high beyond n \cdot p = -1, apart from each other. Measure the two together as one result.
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

N = (1 / 3, 2 / 3, 2 / 3)
U = (2 / 3, -2 / 3, 1 / 3)
V = (2 / 3, 1 / 3, -2 / 3)


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 5).Shape()
    # The slab: a box 30 by 30 along u and v, from n.p = -1 to n.p = 2; its corner is at
    # -15u - 15v - 1n. Ax2 takes the box's z direction (n) and its x direction (u).
    corner = gp_Pnt(*(-15 * U[i] - 15 * V[i] - N[i] for i in range(3)))
    frame = gp_Ax2(corner, gp_Dir(*N), gp_Dir(*U))
    slab = BRepPrimAPI_MakeBox(frame, 30, 30, 3).Shape()
    result = BRepAlgoAPI_Cut(ball, slab)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the ball and the slab")
    return result.Shape()
