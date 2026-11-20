"""A web rib with a straight edge running down over a rounded rim. A stepped shaft of three cylinders about the z axis: radius 20 from z = 0 to 30, radius 14 from
z = 30 to 50, and radius 8 from z = 50 to 70. The big cylinder's top outer edge, the circle ρ = 20, z = 30, is rounded to radius 4: a quarter of a
torus about the circle ρ = 16, z = 26, running from the big cylinder's side at z = 26 to its top
at ρ = 16. The flat ring from ρ = 14 to 16 at z = 30 is the
first shoulder, and the ring from ρ = 8 to 14 at z = 50 the second.

A web rib stands in the corner between the first shoulder and the second cylinder's side. Its
profile, in the half-plane y = 0, x > 0, is the vertical line ρ = 18, which reaches past the shoulder
down to the rounded rim and past the top of the second cylinder; turned about the z axis, it sweeps
the cylinder ρ = 18. The rib is the plate 4 thick, 2 to each side of the plane y = 0 on the +x side,
between the second cylinder's side and that cylinder, from the shaft up to z = 50: it stands on the
shoulder from ρ = 14 to 16, and from ρ = 16 to 18 it fills the space above the rounded rim, down to
the rim, so that its outer face is about 20.5 high at ρ = 18. Its top lies in the plane
z = 50 of the second shoulder, which it continues. The shaft and the rib are one solid."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
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
    shaft = fuse(fuse(cylinder(20, 0, 30), cylinder(14, 30, 50), "the first two stages"),
                 cylinder(8, 50, 70), "the shaft")
    rounding = BRepFilletAPI_MakeFillet(shaft)
    rounding.Add(4, rim_edge(shaft))
    rounding.Build()
    if not rounding.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the big cylinder's rim")
    shaft = rounding.Shape()
    # The web: the slab |y| <= 2 on the +x side, cut to the cylinder the profile line turns into;
    # it starts low enough to reach the rounded rim.
    slab = BRepPrimAPI_MakeBox(gp_Pnt(0, -2, 20), gp_Pnt(20, 2, 50)).Shape()
    rib = common(slab, cylinder(18, 20, 50), "the slab and the rib's cylinder")
    return fuse(shaft, rib, "the rib and the shaft")
