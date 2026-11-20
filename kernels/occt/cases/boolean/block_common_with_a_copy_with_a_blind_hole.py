"""A block and the same block with a blind hole, their common part. A block from (0, 0, 0) to
(10, 10, 6), and the same block less the cylinder of radius 2 about the axis parallel to z through
(5, 5), from z = −1 up to z = 4. Make the part common to the plain block and the holed block."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 6)).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(5, 5, -1), gp_Dir(0, 0, 1)), 2, 5).Shape()
    holed = BRepAlgoAPI_Cut(block, bore)
    if not holed.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the bore")
    plain = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 6)).Shape()
    common = BRepAlgoAPI_Common(plain, holed.Shape())
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the block and the holed block")
    return common.Shape()
