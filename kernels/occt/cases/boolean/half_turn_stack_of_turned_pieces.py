"""A stack of half-turn pieces about the z axis, fused: a half cylinder, a half torus, a half
cone frustum and a half ball, each from 0 to 180 degrees about z."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import (BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere,
                             BRepPrimAPI_MakeTorus)
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

HALF = math.pi


def ax(z):
    return gp_Ax2(gp_Pnt(0, 0, z), gp.DZ_s())


def build():
    solid = BRepPrimAPI_MakeCylinder(ax(0), 7, 2, HALF).Shape()
    parts = [BRepPrimAPI_MakeTorus(ax(2), 5.5, 1, HALF).Shape(),
             BRepPrimAPI_MakeCone(ax(2), 4, 2, 4, HALF).Shape(),
             BRepPrimAPI_MakeSphere(ax(6.5), 2, HALF).Shape()]
    for part in parts:
        fuse = BRepAlgoAPI_Fuse(solid, part)
        if not fuse.IsDone():
            raise Refused("BRepAlgoAPI_Fuse declined a piece")
        solid = fuse.Shape()
    return solid
