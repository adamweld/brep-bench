"""A thick tube pierced through both walls by an oblique cylinder. The tube: the cylinder of radius
5 about the z axis from z = 0 to z = 20, less the coaxial one of radius 3. A cylinder of radius 2
whose axis passes through (1, 0, 10) with direction (3, 1, 2), 40 long, 20 either way from that
point. Cut the tube by it."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    outer = BRepPrimAPI_MakeCylinder(5, 20).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp_Dir(0, 0, 1)), 3, 22).Shape()
    tube = BRepAlgoAPI_Cut(outer, bore)
    if not tube.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not bore the tube")
    n = math.sqrt(14)
    direction = gp_Dir(3 / n, 1 / n, 2 / n)
    start = gp_Pnt(1 - 20 * 3 / n, -20 * 1 / n, 10 - 20 * 2 / n)
    pin = BRepPrimAPI_MakeCylinder(gp_Ax2(start, direction), 2, 40).Shape()
    cut = BRepAlgoAPI_Cut(tube.Shape(), pin)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the tube")
    return cut.Shape()
