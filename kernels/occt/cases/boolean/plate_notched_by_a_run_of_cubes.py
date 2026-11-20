"""A plate notched along an edge by a run of seven cubes. A plate from the origin to (60, 10, 4),
cut one after another by cubes of side 6 centred at x = 8, 11, 14, 17, 28, 40, 52, spanning y from
−3 to 3 and z from −1 to 5."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused

CENTRES = (8, 11, 14, 17, 28, 40, 52)


def build():
    solid = BRepPrimAPI_MakeBox(60, 10, 4).Shape()
    for c in CENTRES:
        cube = BRepPrimAPI_MakeBox(gp_Pnt(c - 3, -3, -1), 6, 6, 6).Shape()
        cut = BRepAlgoAPI_Cut(solid, cube)
        if not cut.IsDone():
            raise Refused(f"BRepAlgoAPI_Cut declined the cube at x = {c}")
        solid = cut.Shape()
    return solid
