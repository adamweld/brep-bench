"""A notched prism, its concave edge between planes at an angle chamfered by its throat. In the xz
plane, the polygon through (0, 0), (10, 0), (10, 3), (4, 3), (1, 7), (0, 7), extruded along +y from
0 to 8. The concave edge x = 4, z = 3 is chamfered by its throat 1/2: equal legs d = √5/2 along the
two faces, the chamfer's face 1/2 from the edge."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.ChFiDS import ChFiDS_ChamfMode
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused
from chamfer import edge_through

THROAT = 0.5


def build():
    outline = BRepBuilderAPI_MakePolygon()
    for x, z in [(0, 0), (10, 0), (10, 3), (4, 3), (1, 7), (0, 7)]:
        outline.Add(gp_Pnt(x, 0, z))
    outline.Close()
    face = BRepBuilderAPI_MakeFace(outline.Wire()).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, 8, 0)).Shape()
    chamfer = BRepFilletAPI_MakeChamfer(prism)
    chamfer.SetMode(ChFiDS_ChamfMode.ChFiDS_ConstThroatChamfer)
    chamfer.Add(THROAT, edge_through(prism, (4, 4, 3)))
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the concave edge by its throat")
    return chamfer.Shape()
