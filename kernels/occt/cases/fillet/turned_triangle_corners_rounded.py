"""A triangular ring turned three quarters of a turn, its three long edges rounded. The right
triangle (ρ, z) = (6, 0), (12, 0), (12, 4) in the half-plane y = 0, x ≥ 0, turned about the z axis
from 0° to 270°; its three long edges rounded to radius 1."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt

from blend import edges_where, rounded

PROFILE = [(6, 0), (12, 0), (12, 4)]  # (ρ, z)


def build():
    outline = BRepBuilderAPI_MakePolygon()
    for rho, z in PROFILE:
        outline.Add(gp_Pnt(rho, 0, z))
    outline.Close()
    ring = BRepPrimAPI_MakeRevol(BRepBuilderAPI_MakeFace(outline.Wire()).Face(), gp.OZ_s(),
                                 3 * math.pi / 2).Shape()
    # the long edges are the arcs at the triangle's corners; their middles are at 135°
    arcs = edges_where(ring, lambda p: any(
        abs(math.hypot(p.X(), p.Y()) - rho) < 1e-7 and abs(p.Z() - z) < 1e-7
        for rho, z in PROFILE))
    assert len(arcs) == 3, len(arcs)
    return rounded(ring, 1, arcs, "the ring's three long edges")
