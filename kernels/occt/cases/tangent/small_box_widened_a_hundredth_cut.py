"""A small square prism (0.0015 square about the z axis, 0.0015 tall) and a copy a hundredth wider
(0.001515 square) between the same planes: the wider copy less it."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused

SIDE = 0.0015
WIDER = 0.001515
HEIGHT = 0.0015


def build():
    wider = BRepPrimAPI_MakeBox(gp_Pnt(-WIDER / 2, -WIDER / 2, 0), gp_Pnt(WIDER / 2, WIDER / 2, HEIGHT)).Shape()
    prism = BRepPrimAPI_MakeBox(gp_Pnt(-SIDE / 2, -SIDE / 2, 0), gp_Pnt(SIDE / 2, SIDE / 2, HEIGHT)).Shape()
    result = BRepAlgoAPI_Cut(wider, prism)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the prism from its wider copy")
    return result.Shape()
