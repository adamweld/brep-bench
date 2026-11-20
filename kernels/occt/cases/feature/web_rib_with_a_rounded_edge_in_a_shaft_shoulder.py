"""A web rib with a rounded edge in a shaft's shoulder. A shaft of two cylinders about the z axis: radius 20 from z = 0 to 30, and on it radius 10 from
z = 30 to 70. The big cylinder's top outer edge, the circle ρ = 20, z = 30, is rounded to radius 2: a
quarter of a torus about the circle ρ = 18, z = 28, running from the big cylinder's side at z = 28 to
its top at ρ = 18. The flat ring from ρ = 10 to 18 at z = 30 is the shoulder.

A web rib stands in the corner between the shoulder and the small cylinder's side. Its profile, in
the half-plane y = 0, x > 0, is the quarter circle of radius 7 about the point (ρ, z) = (10, 30),
from (17, 30) on the shoulder to (10, 37) on the small cylinder's side; turned about the z axis,
the arc sweeps a torus. The rib is the plate 4 thick, 2 to each side of the plane y = 0 on the +x
side, between the shoulder, the small cylinder's side and that torus: the points there within 7 of
the circle ρ = 10, z = 30. Its two flat sides are the planes y = ±2. The rib stands clear of the
rounded edge, and the shaft and the rib are one solid."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from ribs import common, face, fuse, revolve


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
    rounding.Add(2, rim_edge(shaft))
    rounding.Build()
    if not rounding.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the big cylinder's rim")
    shaft = rounding.Shape()
    # The rib's profile arc, the quarter circle about (rho, z) = (10, 30), turned about the axis.
    s = 7 / 2 ** 0.5
    quarter = face([((10, 0, 30), (10, 0, 37)), ((10, 0, 37), (10 + s, 0, 30 + s), (17, 0, 30)),
                    ((17, 0, 30), (10, 0, 30))])
    torus = revolve(quarter, 360)
    slab = BRepPrimAPI_MakeBox(gp_Pnt(0, -2, 30), gp_Pnt(20, 2, 40)).Shape()
    rib = common(slab, torus, "the slab and the turned quarter disc")
    return fuse(shaft, rib, "the rib and the shaft")
