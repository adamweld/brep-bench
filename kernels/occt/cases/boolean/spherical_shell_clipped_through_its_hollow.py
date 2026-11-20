"""A spherical shell, the points between the spheres of radius 3 and 6 about the origin, and a box from (−10, −10, −10) to (10, 10, 1). Take the part of the shell inside the box.

The box is far wider than the shell on every side but the top, where its face z = 1 lies just above the centre and cuts through the hollow as well as the shell's wall: the top is a flat ring, 1 above the centre, between the circle of radius √35 on the outer sphere and the circle of radius √8 on the inner one, and the hollow is open there.
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    outer = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 6).Shape()
    inner = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 3).Shape()
    shell = BRepAlgoAPI_Cut(outer, inner)
    if not shell.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not hollow the ball")
    box = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, -10), gp_Pnt(10, 10, 1)).Shape()
    result = BRepAlgoAPI_Common(shell.Shape(), box)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the shell and the box")
    return result.Shape()
