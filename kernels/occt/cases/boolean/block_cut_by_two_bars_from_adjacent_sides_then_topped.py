"""A block cut from two sides by overlapping bars, then topped. A block from the origin to
(20, 20, 20), cut one after another by the bar from (−5, 3, 2) to (12, 9, 10), the bar from
(8, −5, 6) to (15, 14, 14), and the box from (−5, −5, 12) to (25, 25, 25)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused

CUTTERS = [((-5, 3, 2), (12, 9, 10)),
           ((8, -5, 6), (15, 14, 14)),
           ((-5, -5, 12), (25, 25, 25))]


def build():
    shape = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 20, 20)).Shape()
    for low, high in CUTTERS:
        cutter = BRepPrimAPI_MakeBox(gp_Pnt(*low), gp_Pnt(*high)).Shape()
        cut = BRepAlgoAPI_Cut(shape, cutter)
        if not cut.IsDone():
            raise Refused(f"BRepAlgoAPI_Cut declined the cutter {low}..{high}")
        shape = cut.Shape()
    return shape
