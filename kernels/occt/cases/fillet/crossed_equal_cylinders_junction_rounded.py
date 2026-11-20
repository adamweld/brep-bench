"""Two equal cylinders crossing at right angles, their junction rounded. The cylinder of radius 4
about the x axis from x = -12 to 12 united with the cylinder of radius 4 about the y axis from
y = -12 to 12; the four arcs where their sides meet are rounded to radius 1."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from blend import edges_where, rounded


def on_both_sides(p):
    return abs(p.Y() ** 2 + p.Z() ** 2 - 16) < 1e-6 and abs(p.X() ** 2 + p.Z() ** 2 - 16) < 1e-6


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-12, 0, 0), gp_Dir(1, 0, 0)), 4, 24).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, -12, 0), gp_Dir(0, 1, 0)), 4, 24).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not unite the cylinders")
    cross = fuse.Shape()
    junction = edges_where(cross, on_both_sides)
    if not junction:
        raise Refused("no junction edges")
    return rounded(cross, 1, junction, "the junction")
