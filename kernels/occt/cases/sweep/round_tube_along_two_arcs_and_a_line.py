"""A tube of circular section swept along a path in the xy plane, made of three pieces each running tangent into the next. The path starts at the origin heading along +x:

1. an arc of radius 2 turning left (anticlockwise) a quarter turn, about the centre (0, 2), to (2, 2), where it is heading along +y;
2. an arc of radius 8 turning right (clockwise) an eighth of a turn, about the centre (10, 2), to the point (10 - 4\sqrt{2},\ 2 + 4\sqrt{2}), where it is heading along (1, 1)/\sqrt{2};
3. a straight run of length 3 along that heading.

The section is a disc of radius 1/2, centred on the path and square to it, and the ends are flat. The path is 3π + 3 long.
"""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused

R = math.sqrt(2)


def arc(first, middle, last):
    return BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(first, middle, last).Value()).Edge()


def build():
    # 1: the quarter turn of radius 2 about (0, 2), from the origin to (2, 2).
    start = gp_Pnt(0, 0, 0)
    first = arc(start, gp_Pnt(2 * math.cos(-math.pi / 4), 2 + 2 * math.sin(-math.pi / 4), 0),
                gp_Pnt(2, 2, 0))
    # 2: the eighth of a turn of radius 8 about (10, 2), clockwise from (2, 2) to the point
    # (10 - 4 sqrt 2, 2 + 4 sqrt 2); its middle is at the angle 157.5 degrees.
    angle = math.radians(157.5)
    middle = gp_Pnt(10 + 8 * math.cos(angle), 2 + 8 * math.sin(angle), 0)
    corner = gp_Pnt(10 - 4 * R, 2 + 4 * R, 0)
    second = arc(gp_Pnt(2, 2, 0), middle, corner)
    # 3: the straight run of 3 along (1, 1) / sqrt 2.
    end = gp_Pnt(10 - 4 * R + 3 / R, 2 + 4 * R + 3 / R, 0)
    third = BRepBuilderAPI_MakeEdge(corner, end).Edge()
    path = BRepBuilderAPI_MakeWire()
    for edge in (first, second, third):
        path.Add(edge)
    section = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(start, gp.DX_s()), 0.5)).Edge()
    pipe = BRepOffsetAPI_MakePipeShell(path.Wire())
    pipe.SetMode(gp_Dir(0, 0, 1))  # the path is planar: the section turns about the z axis
    pipe.Add(BRepBuilderAPI_MakeWire(section).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the circle")
    if not pipe.MakeSolid():
        raise Refused("the swept tube did not close into a solid")
    return pipe.Shape()
