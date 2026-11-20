"""An annular sector lofted along a straight spine to a fifth of its size. A ruled loft between
two annular sectors: the first in the plane z = 0, the part of the annulus 2 <= rho <= 4 about the z
axis between 0 and 60 degrees; the second in the plane z = 5, the first scaled by 1/5 about the z axis.
Each edge joined to the matching edge by straight lines, the ends closed with the two sectors."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused

INNER, OUTER = 2.0, 4.0
SPAN = math.radians(60)
HEIGHT = 5.0
SCALE = 1 / 5


def sector(z, scale):
    """The wire of the annular sector at height z, its radii scaled: inner arc, a side out, outer
    arc back, a side in."""
    r0, r1 = INNER * scale, OUTER * scale

    def at(r, a):
        return gp_Pnt(r * math.cos(a), r * math.sin(a), z)

    axis = gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1))
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Circ(axis, r0), 0, SPAN).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(at(r0, SPAN), at(r1, SPAN)).Edge())
    wire.Add(TopoDS.Edge(BRepBuilderAPI_MakeEdge(gp_Circ(axis, r1), 0, SPAN).Edge().Reversed()))
    wire.Add(BRepBuilderAPI_MakeEdge(at(r1, 0), at(r0, 0)).Edge())
    return wire.Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled
    loft.AddWire(sector(0, 1))
    loft.AddWire(sector(HEIGHT, SCALE))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the two sectors")
    return loft.Shape()
