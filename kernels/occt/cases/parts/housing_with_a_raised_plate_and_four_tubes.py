"""A housing: a rounded base 80 by 50 by 10 with a hole of radius 12, a raised plate 60 by 30 by 4, and
four tubes of outer radius 5 and bore 3 standing on the base."""
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

import math


def rounded_base():
    base = box(-40, -25, 0, 40, 25, 10)
    # Round the four vertical edges by cutting the corner and adding the quarter cylinder back.
    for sx in (-1, 1):
        for sy in (-1, 1):
            cx, cy = sx * 32, sy * 17
            lo_x, hi_x = (cx, 40) if sx > 0 else (-40, cx)
            lo_y, hi_y = (cy, 25) if sy > 0 else (-25, cy)
            corner = box(lo_x, lo_y, 0, hi_x, hi_y, 10)
            base = run(BRepAlgoAPI_Cut(base, corner), "BRepAlgoAPI_Cut")
            disc = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(cx, cy, 0), gp.DZ_s()), 8, 10).Shape()
            quarter = run(BRepAlgoAPI_Common(disc, corner), "BRepAlgoAPI_Common")
            base = run(BRepAlgoAPI_Fuse(base, quarter), "BRepAlgoAPI_Fuse")
    return base


def cyl(x, y, z0, r, h):
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, y, z0), gp.DZ_s()), r, h).Shape()


def build():
    shape = run(BRepAlgoAPI_Fuse(rounded_base(), box(-30, -15, 10, 30, 15, 14)), "BRepAlgoAPI_Fuse")
    shape = run(BRepAlgoAPI_Cut(shape, cyl(0, 0, -1, 12, 16)), "BRepAlgoAPI_Cut")
    for x in (-34, 34):
        for y in (-19, 19):
            shape = run(BRepAlgoAPI_Fuse(shape, cyl(x, y, 10, 5, 15)), "BRepAlgoAPI_Fuse")
    for x in (-34, 34):
        for y in (-19, 19):
            shape = run(BRepAlgoAPI_Cut(shape, cyl(x, y, -1, 3, 27)), "BRepAlgoAPI_Cut")
    return shape
