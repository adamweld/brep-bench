"""A web rib with a quarter-arc edge reaching over a shaft's rounded rim. A shaft of two cylinders about the z axis: radius 20 from z = 0 to 30, and on it radius 10 from
z = 30 to 70. The big cylinder's top outer edge, the circle ρ = 20, z = 30, is rounded to radius 4: a quarter of a
torus about the circle ρ = 16, z = 26, running from the big cylinder's side at z = 26 to its top
at ρ = 16. The flat ring from ρ = 10 to 16 at z = 30 is the shoulder.

A web rib stands in the corner between the shoulder and the small cylinder's side. Its profile, in
the half-plane y = 0, x > 0, is the circle of radius 7 about the point (ρ, z) = (10, 30), taken from
(10, 37) on the small cylinder's side round through (17, 30) and on until it meets the shaft again,
just below z = 30 over the rounded rim; turned about the z axis, that circle sweeps a torus. The rib
is the plate 4 thick, 2 to each side of the plane y = 0 on the +x side, of all the points within 7 of
the circle ρ = 10, z = 30 that the shaft does not already hold. So it fills the corner under the arc,
reaches out over the round, where it ends in a thin tip at ρ = 17, and its two flat sides are the
planes y = ±2. The shaft and the rib are one solid."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeTorus
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from ribs import common, fuse


def cylinder(radius, z0, z1):
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, z0), gp_Dir(0, 0, 1)), radius, z1 - z0).Shape()


def rim_edge(shape):
    """The circle of radius 20 at z = 30, where the big cylinder's side meets its top."""
    explorer = TopExp_Explorer(shape, TopAbs_EDGE)
    while explorer.More():
        edge = TopoDS.Edge(explorer.Current())
        curve = BRepAdaptor_Curve(edge)
        p = curve.Value((curve.FirstParameter() + curve.LastParameter()) / 2)
        if abs(p.Z() - 30) < 1e-9 and abs((p.X() ** 2 + p.Y() ** 2) ** 0.5 - 20) < 1e-9:
            return edge
        explorer.Next()
    raise Refused("the shaft has no rim at z = 30")


def build():
    shaft = fuse(cylinder(20, 0, 30), cylinder(10, 30, 70), "the shaft")
    rounding = BRepFilletAPI_MakeFillet(shaft)
    rounding.Add(4, rim_edge(shaft))
    rounding.Build()
    if not rounding.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the big cylinder's rim")
    shaft = rounding.Shape()
    # The torus the rib's circle sweeps, cut to the slab |y| <= 2 on the +x side.
    torus = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 30), gp_Dir(0, 0, 1)), 10, 7).Shape()
    slab = BRepPrimAPI_MakeBox(gp_Pnt(0, -2, 20), gp_Pnt(20, 2, 40)).Shape()
    rib = common(slab, torus, "the slab and the torus")
    return fuse(shaft, rib, "the rib and the shaft")
