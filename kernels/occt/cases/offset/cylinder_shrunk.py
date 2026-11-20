"""A cylinder lofted between two circles, shrunk by 1. The cylinder on the z axis made by a ruled
loft between the circle of radius 5 at z = 0 and the equal circle at z = 8, closed by the two disks
they bound, shrunk by 1."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused
from offset import offset

RADIUS, HEIGHT = 5, 8
SHRINKAGE = 1


def circle(z):
    edge = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), RADIUS))
    return BRepBuilderAPI_MakeWire(edge.Edge()).Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled between the sections
    loft.AddWire(circle(0))
    loft.AddWire(circle(HEIGHT))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the cylinder")
    return offset(loft.Shape(), -SHRINKAGE)
