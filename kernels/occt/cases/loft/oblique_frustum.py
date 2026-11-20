"""A frustum of an oblique cone. A circle of radius 6 about the origin in z = 0 and one of radius
3 about (8, 0, 10) in z = 10, joined by straight lines between points at the same angle, and
closed by the two discs."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused


def circle(centre, radius):
    ring = gp_Circ(gp_Ax2(gp_Pnt(*centre), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), radius)
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(ring).Edge()).Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # solid, ruled
    loft.AddWire(circle((0, 0, 0), 6))
    loft.AddWire(circle((8, 0, 10), 3))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the two circles")
    return loft.Shape()
