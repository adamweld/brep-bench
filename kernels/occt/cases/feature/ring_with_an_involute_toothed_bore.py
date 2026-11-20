"""A ring whose bore is a twelve-tooth gear outline: the cylinder of radius 20 about the z axis,
z from 0 to 8, less the gear outline of the spur-gear case extruded right through it."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused, other_case


def build():
    gear = other_case("feature/spur-gear-of-twelve-involute-teeth")
    face = gear.outline()
    down = gp_Trsf()
    down.SetTranslation(gp_Vec(0, 0, -1))
    face = BRepBuilderAPI_Transform(face, down, True).Shape()
    cutter = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, 10)).Shape()  # z from -1 to 9
    blank = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 20, 8).Shape()
    cut = BRepAlgoAPI_Cut(blank, cutter)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the toothed bore")
    return cut.Shape()
