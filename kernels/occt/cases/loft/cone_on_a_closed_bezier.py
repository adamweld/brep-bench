r"""A cone on a Bezier teardrop: a ruled loft from a closed Bezier curve to a point, capped flat.
"""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeVertex, BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.Geom import Geom_BezierCurve, Geom_BSplineCurve
from OCP.collections import Array1_double, Array1_gp_Pnt, Array1_int
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused


def vertex(p):
    return BRepBuilderAPI_MakeVertex(gp_Pnt(*p)).Vertex()


def cone_loft(wire, apex, name):
    """A ruled ThruSections from the closed wire to the vertex, as a solid (the wire is capped)."""
    thru = BRepOffsetAPI_ThruSections(True, True)
    thru.AddWire(wire)
    thru.AddVertex(vertex(apex))
    thru.Build()
    if not thru.IsDone():
        raise Refused(f"BRepOffsetAPI_ThruSections made no solid for {name}")
    return thru.Shape()


def build():
    poles = Array1_gp_Pnt(1, 4)
    for i, p in enumerate([(-3, 0, 0), (5, -6, 0), (5, 6, 0), (-3, 0, 0)], 1):
        poles.SetValue(i, gp_Pnt(*p))
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()).Wire()
    return cone_loft(wire, (1, 0.5, 4), "the cone on a Bezier curve")
