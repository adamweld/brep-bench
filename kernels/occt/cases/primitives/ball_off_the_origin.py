"""A ball off the origin: a solid ball of radius 5 centred on the point (3, -2, 4)."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

CENTRE = gp_Pnt(3, -2, 4)
RADIUS = 5


def build():
    return BRepPrimAPI_MakeSphere(CENTRE, RADIUS).Shape()
