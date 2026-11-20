"""A drafted post on a dome, its foot rounded. A hemisphere of radius 15 (z ≥ 0) united with a post
of revolution, radius 4 − (z − 15)·tan 3° from z = 0 to z = 22, the cylinder of radius 4 drafted 3°
about z = 15; the circle where the cone leaves the sphere rounded to radius 1.5, adding material."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeSphere
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

DOME, FOOT_RADIUS, TOP = 15, 1.5, 22
LEAN = math.tan(math.radians(3))


def build():
    dome = BRepPrimAPI_MakeSphere(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), DOME,
                                  0, math.pi / 2).Shape()  # a hemisphere with its flat face
    post = BRepPrimAPI_MakeCone(4 + DOME * LEAN, 4 - (TOP - DOME) * LEAN, TOP).Shape()
    fuse = BRepAlgoAPI_Fuse(dome, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the post")
    part = fuse.Shape()
    fillet = BRepFilletAPI_MakeFillet(part)
    edges = TopExp_Explorer(part, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        if (curve.GetType() == GeomAbs_CurveType.GeomAbs_Circle
                and 4 < curve.Circle().Radius() < 4.1 and 14 < curve.Circle().Location().Z() < 15):
            fillet.Add(FOOT_RADIUS, edge)
        edges.Next()
    assert fillet.NbContours() == 1, f"{fillet.NbContours()} circles where the post leaves"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the post's foot")
    return fillet.Shape()
