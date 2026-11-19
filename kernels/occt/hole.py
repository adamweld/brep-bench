"""The OCCT implementations' one way of making a hole: its cutting tool, revolved from a half section.

A drawing of a hole gives its half section: the radius at each depth, down its axis. `cutter`
takes that outline, as (radius, depth) corners from the mouth inward, closes it on the axis, and
revolves it a full turn about the axis, which is the solid a hole's tools take away.
"""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp_Ax1, gp_Ax2, gp_Dir, gp_Pnt


def cutter(mouth, into, section):
    """The solid of revolution about the axis through `mouth` along `into` (into the material),
    whose half section has the corners `section`, (radius, depth) pairs, depth measured from
    `mouth` along `into`. The section runs from the axis out and back to it."""
    frame = gp_Ax2(gp_Pnt(*mouth), gp_Dir(*into))
    origin, down, out = frame.Location(), frame.Direction(), frame.XDirection()
    outline = BRepBuilderAPI_MakePolygon()
    for radius, depth in section:
        outline.Add(gp_Pnt(origin.X() + radius * out.X() + depth * down.X(),
                           origin.Y() + radius * out.Y() + depth * down.Y(),
                           origin.Z() + radius * out.Z() + depth * down.Z()))
    outline.Close()
    face = BRepBuilderAPI_MakeFace(outline.Wire()).Face()
    return BRepPrimAPI_MakeRevol(face, gp_Ax1(origin, down)).Shape()
