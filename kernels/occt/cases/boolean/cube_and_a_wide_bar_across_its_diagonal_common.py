"""A cube and a wide bar turned across its diagonal from just outside a corner, their common part.
A cube from (0, 0, 0) to (10, 10, 10), and a bar 6 wide, 20 long and 10 high on the xy plane, one
short end centred on (−1, −1), its long sides running along (1, 1). Make the solid common to
both."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    h = 3 * math.sqrt(2) / 2
    # the box's x runs along (1, 1), its y along (−1, 1)/√2; the corner is the end's right side
    bar = BRepPrimAPI_MakeBox(gp_Ax2(gp_Pnt(-1 + h, -1 - h, 0), gp.DZ_s(), gp_Dir(1, 1, 0)),
                              20, 6, 10).Shape()
    common = BRepAlgoAPI_Common(cube, bar)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the bar")
    return common.Shape()
