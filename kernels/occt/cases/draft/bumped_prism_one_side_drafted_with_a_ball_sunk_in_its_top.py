"""A prism with a half-round bump, one side drafted, with a ball sunk in its top. The rectangle
0 <= x <= 20, 0 <= y <= 12 with the half disk of radius 4 about (10, 12) on its side y >= 12,
extruded to z = 10. The side x = 20 drafted 5 degrees, pull +z, neutral plane z = 10. A ball of
radius 3 centred at (6, 4, 12) fused on."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused, face_through
from draft import drafted

HEIGHT = 10
DRAFT = 5  # degrees, inward as the face rises


def build():
    def p(x, y):
        return gp_Pnt(x, y, 0)

    wire = BRepBuilderAPI_MakeWire()
    for a, b in [((0, 0), (20, 0)), ((20, 0), (20, 12)), ((20, 12), (14, 12))]:
        wire.Add(BRepBuilderAPI_MakeEdge(p(*a), p(*b)).Edge())
    # the half round from (14, 12) over the top to (6, 12): the circle about (10, 12), 0 to 180
    # degrees traversed backwards
    circle = gp_Circ(gp_Ax2(p(10, 12), gp_Dir(0, 0, 1)), 4)
    arc = BRepBuilderAPI_MakeEdge(circle, 0, math.pi).Edge()
    wire.Add(arc)
    for a, b in [((6, 12), (0, 12)), ((0, 12), (0, 0))]:
        wire.Add(BRepBuilderAPI_MakeEdge(p(*a), p(*b)).Edge())
    if not wire.IsDone():
        raise Refused("BRepBuilderAPI_MakeWire could not close the outline")
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, HEIGHT)).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(6, 4, 12), 3).Shape()
    fuse = BRepAlgoAPI_Fuse(prism, ball)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the ball")
    part = fuse.Shape()
    side = face_through(part, (20, 6, HEIGHT / 2))
    return drafted(part, [side], DRAFT, neutral=(0, 0, HEIGHT))
