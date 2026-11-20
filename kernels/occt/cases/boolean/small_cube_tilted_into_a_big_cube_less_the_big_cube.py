"""A small cube tilted 45 degrees into a big cube, less the big cube."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Dir, gp_Pnt, gp_Trsf

from bench import Refused


def box(a, b):
    return BRepPrimAPI_MakeBox(gp_Pnt(*a), gp_Pnt(*b)).Shape()


def post(radius, z0, z1):
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, z0), gp.DZ_s()), radius, z1 - z0).Shape()


def turned(shape, about, direction, angle):
    t = gp_Trsf()
    t.SetRotation(gp_Ax1(gp_Pnt(*about), gp_Dir(*direction)), angle)
    return BRepBuilderAPI_Transform(shape, t, True).Shape()


def cut(first, second):
    """The second less the first."""
    op = BRepAlgoAPI_Cut(second, first)
    if not op.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined")
    return op.Shape()



def build():
    big = box((0, 0, 0), (10, 10, 10))
    small = turned(box((3, 5, 9), (7, 9, 13)), (0, 5, 9), (1, 0, 0), math.pi / 4)
    return cut(big, small)
