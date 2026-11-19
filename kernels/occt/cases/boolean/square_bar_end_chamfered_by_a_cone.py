"""A square bar's end chamfered by a cone. A square bar, the box from (−10, −10, 0) to (10, 10, 12),
and a cone about the z axis that passes through the circle of radius 13 in the bar's end face, z
= 12, and makes 30° with that face, widening downward: its radius at height z is 13 + √3(12 − z).
Their common part: the bar's end turned with a 30° chamfer, as a square nut's corners are.

The cone cuts only the upper ends of the bar's four vertical edges. The end face is the square
less its four corners outside the circle of radius 13, and each side face loses a sliver above
the hyperbola in which the cone cuts it, down to z = 12 − (10√2 − 13)/√3 ≈ 11.340 at the bar's
edges."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

HALF, HEIGHT, RIM = 10, 12, 13


def build():
    bar = BRepPrimAPI_MakeBox(gp_Pnt(-HALF, -HALF, 0), gp_Pnt(HALF, HALF, HEIGHT)).Shape()
    # the cone at 30° to the end face through the circle of radius 13 on it, from z = 0 to the end
    spread = HEIGHT / math.tan(math.radians(30))
    cone = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)),
                                RIM + spread, RIM, HEIGHT).Shape()
    common = BRepAlgoAPI_Common(bar, cone)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the bar and the cone")
    return common.Shape()
