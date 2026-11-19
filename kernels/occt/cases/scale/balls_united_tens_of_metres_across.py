"""Two balls tens of metres across, united. A ball of radius 50000 centred on the origin and a ball
of radius 40000 centred on (20000, 30000, 60000), 70000 from the first. Unite them."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 50000).Shape()
    second = BRepPrimAPI_MakeSphere(gp_Pnt(20000, 30000, 60000), 40000).Shape()
    union = BRepAlgoAPI_Fuse(first, second)
    if not union.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two balls")
    return union.Shape()
