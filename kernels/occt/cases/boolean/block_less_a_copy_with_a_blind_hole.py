"""A block less the same block with a blind hole: the plug of the hole. A block from (0, 0, 0) to
(10, 10, 6), and a second block, the same block with a blind hole: the block less the cylinder of
radius 2 about the axis parallel to z through (5, 5), from z = −1 up to z = 4, so the hole opens
in the bottom face and ends flat 4 up. Cut the holed block from the plain block. All their faces
coincide except the hole; what is left is the plug that filled it: the cylinder of radius 2 about
that axis from z = 0 to z = 4."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 6)).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(5, 5, -1), gp_Dir(0, 0, 1)), 2, 5).Shape()
    holed = BRepAlgoAPI_Cut(block, bore)
    if not holed.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the bore")
    cut = BRepAlgoAPI_Cut(BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 6)).Shape(), holed.Shape())
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the block and the holed block")
    return cut.Shape()
