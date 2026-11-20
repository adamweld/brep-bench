"""A web rib with a cylindrical edge in the corner of a stepped shaft. A stepped shaft of three cylinders about the z axis: radius 20 from z = 0 to 30, radius 14 from
z = 30 to 50, and radius 8 from z = 50 to 70. The ring from ρ = 14 to 20 at z = 30 is the first
shoulder, and the ring from ρ = 8 to 14 at z = 50 the second.

A web rib stands in the corner between the first shoulder and the second cylinder's side. Its
profile, in the half-plane y = 0, x > 0, is the vertical line ρ = 18; turned about the z axis, the
line sweeps the cylinder ρ = 18. The rib is the plate 4 thick, 2 to each side of the plane y = 0 on
the +x side, between the second cylinder's side and that cylinder, from the shoulder at z = 30 up to
z = 50. So it meets the shoulder from ρ = 14 to ρ = 18 and the second cylinder's side from z = 30 to
z = 50; its top lies in the plane z = 50 of the second shoulder, which it continues; its two flat
sides are the planes y = ±2, and its outer face is a piece of the cylinder ρ = 18. The shaft and the
rib are one solid."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from ribs import common, fuse


def cylinder(radius, z0, z1):
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, z0), gp_Dir(0, 0, 1)), radius, z1 - z0).Shape()


def build():
    shaft = fuse(fuse(cylinder(20, 0, 30), cylinder(14, 30, 50), "the first two stages"),
                 cylinder(8, 50, 70), "the shaft")
    # The web: the slab |y| <= 2 on the +x side, cut to the cylinder the profile line turns into.
    slab = BRepPrimAPI_MakeBox(gp_Pnt(0, -2, 30), gp_Pnt(20, 2, 50)).Shape()
    rib = common(slab, cylinder(18, 30, 50), "the slab and the rib's cylinder")
    return fuse(shaft, rib, "the rib and the shaft")
