"""A block turned 30 degrees standing out of a cube, less the cube."""
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
    cube = box((0, 0, 0), (10, 10, 10))
    block = turned(box((1, 1, 6), (9, 9, 16)), (5, 5, 0), (0, 0, 1), math.pi / 6)
    return cut(cube, block)
