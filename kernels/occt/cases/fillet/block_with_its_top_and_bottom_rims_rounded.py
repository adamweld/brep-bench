"""A block with the rims of its top and bottom rounded, its upright edges sharp. The block
[-10, 10] x [-10, 10] x [-5, 5]; the eight edges bounding its top and bottom faces rounded with
radius 2, the four upright edges left sharp."""
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, -5), gp_Pnt(10, 10, 5)).Shape()
    fillet = BRepFilletAPI_MakeFillet(block)
    edges = TopExp_Explorer(block, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        a = curve.Value(curve.FirstParameter())
        b = curve.Value(curve.LastParameter())
        if abs(a.Z() - b.Z()) < 1e-9:  # not an upright edge
            fillet.Add(2, edge)
        edges.Next()
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet declined the eight rim edges")
    return fillet.Shape()
