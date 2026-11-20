"""A closed ruled loft round a diamond-shaped loop. Four circles of radius 5 centred 30 u_k from
the origin (u_k = (cos 90k, sin 90k, 0)), each in the vertical plane through u_k; the point at angle
theta of each joined to the point at the same angle of the next, round to the first again. Each
quarter is a ruled loft of two circles; the four shells are sewn into one solid."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeSolid,
                                BRepBuilderAPI_MakeWire, BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused


def circle(k):
    """Circle k: centre 30 u_k, radius 5, angle 0 along u_k, angle 90 degrees straight up."""
    c, s = round(math.cos(math.pi * k / 2)), round(math.sin(math.pi * k / 2))
    axis = gp_Ax2(gp_Pnt(30 * c, 30 * s, 0), gp_Dir(s, -c, 0), gp_Dir(c, s, 0))
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(gp_Circ(axis, 5)).Edge()).Wire()


def build():
    sewing = BRepBuilderAPI_Sewing(1e-6)
    for k in range(4):
        loft = BRepOffsetAPI_ThruSections(False, True)  # a shell, ruled
        loft.AddWire(circle(k))
        loft.AddWire(circle((k + 1) % 4))
        loft.Build()
        if not loft.IsDone():
            raise Refused("BRepOffsetAPI_ThruSections could not loft two circles of the loop")
        sewing.Add(loft.Shape())
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused("BRepBuilderAPI_Sewing made no shell of the four lofts")
    make = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells.Current()))
    if not make.IsDone():
        raise Refused("BRepBuilderAPI_MakeSolid made no solid of the sewn shell")
    solid = make.Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid
