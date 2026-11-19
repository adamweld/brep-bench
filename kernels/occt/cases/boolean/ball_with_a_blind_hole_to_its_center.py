"""A ball with a blind hole drilled from below to its center. A ball of radius 10 about the origin less
the cylinder of radius 3 about the z axis from z = −15 to z = 0."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def box(x0, y0, z0, x1, y1, z1):
    return BRepPrimAPI_MakeBox(gp_Pnt(x0, y0, z0), gp_Pnt(x1, y1, z1)).Shape()


def run(op, name):
    if not op.IsDone():
        raise Refused(f"{name} declined")
    return op.Shape()


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 10).Shape()
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -15), gp.DZ_s()), 3, 15).Shape()
    return run(BRepAlgoAPI_Cut(ball, drill), "BRepAlgoAPI_Cut")
