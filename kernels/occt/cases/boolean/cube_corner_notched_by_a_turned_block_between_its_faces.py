"""A cube's vertical edge notched away by a turned block that lies between its top and base. The cube from (0, 0, 0) to (10, 10, 10), cut by a block turned about the vertical:
its footprint is 5 by 5, from the corner (10, 7) with edges along (4/5, 3/5) and (−3/5, 4/5),
standing from z = 5/2 to z = 5/25."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    # the box's x runs along the placement's x direction, its y along z × x = (−3/5, 4/5, 0)
    block = BRepPrimAPI_MakeBox(gp_Ax2(gp_Pnt(10, 7, 5/2), gp.DZ_s(), gp_Dir(4, 3, 0)),
                                5, 5, 5).Shape()
    cut = BRepAlgoAPI_Cut(cube, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the block")
    return cut.Shape()
