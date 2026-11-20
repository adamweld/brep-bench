"""A ring with a quarter-ellipse section turned three quarters of a turn, its three circular edges
rounded. The region of the half-plane y = 0 bounded by (3, 0)-(6, 0), the quarter of the ellipse
about (3, 0) with semi-axes 3 and 2 from (6, 0) to (3, 2), and (3, 2)-(3, 0), turned 270° about the
z axis; the three circular edges rounded to 1/2."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from blend import edges_where, rounded


def build():
    # the plane y = 0 seen with local x = +x and local y = +z
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(3, 0, 0), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0)), 3, 2)
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(3, 0, 0), gp_Pnt(6, 0, 0)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(ellipse, 0, math.pi / 2).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(3, 0, 2), gp_Pnt(3, 0, 0)).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    ring = BRepPrimAPI_MakeRevol(face, gp.OZ_s(), 3 * math.pi / 2).Shape()
    corners = [(6, 0), (3, 2), (3, 0)]
    arcs = edges_where(ring, lambda p: any(
        abs(math.hypot(p.X(), p.Y()) - rho) < 1e-7 and abs(p.Z() - z) < 1e-7 for rho, z in corners))
    assert len(arcs) == 3, len(arcs)
    return rounded(ring, 0.5, arcs, "the ring's three circular edges")
