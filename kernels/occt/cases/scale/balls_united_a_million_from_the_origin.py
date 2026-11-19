"""Two balls united a million from the origin. A ball of radius 5 centred on (600000, −800000, 0)
and a ball of radius 4 centred on (600002, −799997, 6), 7 from the first along (2, 3, 6)/7.
Unite them."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeSphere(gp_Pnt(600000, -800000, 0), 5).Shape()
    second = BRepPrimAPI_MakeSphere(gp_Pnt(600002, -799997, 6), 4).Shape()
    union = BRepAlgoAPI_Fuse(first, second)
    if not union.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two balls")
    return union.Shape()
