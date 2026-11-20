"""A ruled loft through four circles stepped round a distant axis. Four circles of radius 2 with centers
(10 cos a, 10 sin a, 0), a = 0, 30, 60, 90 degrees, each in the vertical plane through the z axis, joined
by a ruled loft and closed by flat discs."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def box(x0, y0, z0, x1, y1, z1):
    return BRepPrimAPI_MakeBox(gp_Pnt(x0, y0, z0), gp_Pnt(x1, y1, z1)).Shape()


def run(op, name):
    if not op.IsDone():
        raise Refused(f"{name} declined")
    return op.Shape()

import math


from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Circ, gp_Dir


def section(k):
    a = math.radians(30 * k)
    centre = gp_Pnt(10 * math.cos(a), 10 * math.sin(a), 0)
    # Normal (sin a, -cos a, 0), x direction (cos a, sin a, 0): the circle's y direction is +z.
    axes = gp_Ax2(centre, gp_Dir(math.sin(a), -math.cos(a), 0), gp_Dir(math.cos(a), math.sin(a), 0))
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(gp_Circ(axes, 2)).Edge()).Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)   # a solid, ruled
    for k in range(4):
        loft.AddWire(section(k))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections declined the ruled loft")
    return loft.Shape()
