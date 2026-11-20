r"""A three-sided pyramid with its apex off the middle of its base: a ruled loft from a triangle to a point, capped flat.
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
    poly = BRepBuilderAPI_MakePolygon()
    for p in [(4, 0, 0), (-2, 2 * math.sqrt(3), 0), (-2, -2 * math.sqrt(3), 0)]:
        poly.Add(gp_Pnt(*p))
    poly.Close()
    return cone_loft(poly.Wire(), (1, 0.5, 5), "the pyramid")
