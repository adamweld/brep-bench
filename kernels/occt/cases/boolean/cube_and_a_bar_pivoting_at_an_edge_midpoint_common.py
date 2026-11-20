"""A cube and a bar turned diagonally from the midpoint of one of its edges, their common part. A
cube from (0, 0, 0) to (10, 10, 10), and a bar 2 wide, 9 long and 10 high on the xy plane, one short
end centred on (5, 0), its long sides running along (1, 1). Make the solid common to both."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    h = math.sqrt(2) / 2
    # the box's x runs along (1, 1), its y along z × x = (−1, 1)/√2; its corner is the end's
    # right-hand corner, (5, 0) less the half width along y
    bar = BRepPrimAPI_MakeBox(gp_Ax2(gp_Pnt(5 + h, -h, 0), gp.DZ_s(), gp_Dir(1, 1, 0)),
                              9, 2, 10).Shape()
    common = BRepAlgoAPI_Common(cube, bar)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the bar")
    return common.Shape()
