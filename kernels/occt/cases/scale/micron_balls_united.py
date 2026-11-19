"""Two balls a few microns across, united. A ball of radius 0.005 centred on the origin and a ball
of radius 0.004 centred on (0.002, 0.003, 0.006), 0.007 from the first. Unite them."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 0.005).Shape()
    second = BRepPrimAPI_MakeSphere(gp_Pnt(0.002, 0.003, 0.006), 0.004).Shape()
    union = BRepAlgoAPI_Fuse(first, second)
    if not union.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two balls")
    return union.Shape()
