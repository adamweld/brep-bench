"""A block less the hollow block whose hollow it fills. A hollow block: the cube [0, 10]^3 with the box
[2, 6] x [3, 8] x [4, 9] hollowed out of it. The filler is that box itself. Cut the hollow block away from
the filler."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from OCP.gp import gp_Pnt as P


def build():
    outer = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    inner = BRepPrimAPI_MakeBox(P(2, 3, 4), P(6, 8, 9)).Shape()
    hollow = BRepAlgoAPI_Cut(outer, inner)
    if not hollow.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined to hollow the block")
    filler = BRepPrimAPI_MakeBox(P(2, 3, 4), P(6, 8, 9)).Shape()
    cut = BRepAlgoAPI_Cut(filler, hollow.Shape())
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the filler and the hollow block")
    return cut.Shape()
