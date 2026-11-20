"""A wedge with a smaller wedge cut from its base. The hull of a rectangle 8 by 6 in z = 0 and a ridge
from (2, 3, 4) to (6, 3, 4), less the hull of a rectangle 4 by 3 on the same floor and the ridge from
(3, 3, 3) to (5, 3, 3); the inner one open through the base."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from bench import Refused


def flat(corners):
    poly = BRepBuilderAPI_MakePolygon()
    for corner in corners:
        poly.Add(gp_Pnt(*corner))
    poly.Close()
    return BRepBuilderAPI_MakeFace(poly.Wire(), True).Face()


def wedge(x0, x1, y0, y1, ridge_x0, ridge_x1, ridge_z, z0=0):
    """The ridged wedge on the rectangle [x0, x1] × [y0, y1] at z0, its ridge from ridge_x0 to ridge_x1
    along the middle, sewn from its five faces."""
    ym = (y0 + y1) / 2
    a, b, c, d = (x0, y0, z0), (x1, y0, z0), (x1, y1, z0), (x0, y1, z0)
    r0, r1 = (ridge_x0, ym, ridge_z), (ridge_x1, ym, ridge_z)
    sewing = BRepBuilderAPI_Sewing(1e-6)
    for face in ([a, d, c, b], [a, b, r1, r0], [b, c, r1], [c, d, r0, r1], [d, a, r0]):
        sewing.Add(flat(face))
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused("BRepBuilderAPI_Sewing made no shell of the wedge's faces")
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells.Current())).Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid


def build():
    outer = wedge(0, 8, 0, 6, 2, 6, 4)
    # the inner wedge is extended below the floor along its own slopes so that it cuts clean through
    # the base: its sides run on, one unit lower and wider
    inner = wedge(2 - 1 / 3, 6 + 1 / 3, 1.5 - 0.5, 4.5 + 0.5, 3, 5, 3, z0=-1)
    cut = BRepAlgoAPI_Cut(outer, inner)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two wedges")
    return cut.Shape()
