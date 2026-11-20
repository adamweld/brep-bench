"""A prism with a rounded corner, its far side drafted, under a half ball. The right triangle (0, 0),
(20, 0), (0, 12) extruded to z = 10, the edge at the origin rounded to radius 2. Its hypotenuse
drafted 5 degrees, pull +z, neutral plane z = 10. A half ball of radius 3 fused on the top, centred
at (6, 4.5, 10)."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism, BRepPrimAPI_MakeSphere
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused, face_through
from draft import drafted

HEIGHT = 10
DRAFT = 5  # degrees, inward as the face rises


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in [(0, 0), (20, 0), (0, 12)]:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, HEIGHT)).Shape()
    rounds = BRepFilletAPI_MakeFillet(prism)
    edges = TopExp_Explorer(prism, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        a, b = curve.Value(curve.FirstParameter()), curve.Value(curve.LastParameter())
        if max(abs(a.X()), abs(a.Y()), abs(b.X()), abs(b.Y())) < 1e-9:
            rounds.Add(2, edge)
        edges.Next()
    rounds.Build()
    if not rounds.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the corner")
    body = rounds.Shape()
    dome = BRepPrimAPI_MakeSphere(gp_Ax2(gp_Pnt(6, 4.5, HEIGHT), gp_Dir(0, 0, 1)), 3,
                                  0, math.pi / 2).Shape()
    fuse = BRepAlgoAPI_Fuse(body, dome)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the half ball")
    part = fuse.Shape()
    side = face_through(part, (10, 6, HEIGHT / 2))
    return drafted(part, [side], DRAFT, neutral=(0, 0, HEIGHT))
