"""A frustum lofted between two rounded squares of NURBS, shelled with its narrow top open. The
solid made by a ruled loft between the square with corners (±3, ±3, 8) rounded by quarter circles
of radius 1, as one closed rational B-spline curve, and that curve scaled by 2 about (0, 0, 8),
moved down to z = 0 and shifted by (1, 0.5), closed by the two flat faces, shelled inward to a
wall 0.5 thick, open at its top face."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire,
                                BRepBuilderAPI_NurbsConvert)
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused
from shell import face_through, shell

HEIGHT = 8
THICKNESS = 0.5


def rounded_square(cx, cy, z, half, radius):
    """The square of half-side `half` about (cx, cy, z), its corners rounded: four lines and four
    quarter circles, converted to NURBS."""
    wire = BRepBuilderAPI_MakeWire()
    straight = half - radius
    for turn in range(4):
        def at(x, y):
            for _ in range(turn):
                x, y = -y, x
            return gp_Pnt(cx + x, cy + y, z)
        wire.Add(BRepBuilderAPI_MakeEdge(at(half, -straight), at(half, straight)).Edge())
        arc = gp_Circ(gp_Ax2(at(straight, straight), gp_Dir(0, 0, 1)), radius)
        start = turn * math.pi / 2
        wire.Add(BRepBuilderAPI_MakeEdge(arc, start, start + math.pi / 2).Edge())
    return TopoDS.Wire(BRepBuilderAPI_NurbsConvert(wire.Wire(), True).Shape())


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled between the sections
    loft.AddWire(rounded_square(1, 0.5, 0, 6, 2))
    loft.AddWire(rounded_square(0, 0, HEIGHT, 3, 1))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the rounded squares")
    solid = loft.Shape()
    return shell(solid, [face_through(solid, (0, 0, HEIGHT))], THICKNESS)
