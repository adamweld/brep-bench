"""A web rib between a drum's top and a square boss, its edge a cone. A drum, the cylinder of radius 20 about the z axis from z = 0 to 10, with a square boss on its top:
the box |x| ≤ 8, |y| ≤ 8, from z = 10 to 40.

A web rib stands on the drum's top against the boss's side x = 8. Its profile, in the half-plane
y = 0, x > 0, is the straight line from (ρ, z) = (20, 10) on the drum's rim up to (8, 30) on the
boss's side; turned about the z axis, the line sweeps a cone, ρ = 20 − 0.6(z − 10). The rib is the
plate 4 thick, 2 to each side of the plane y = 0, between the drum's top, the boss's side and that
cone, so it is 0 high at the rim and 20 high at the boss. Its two flat sides are the planes y = ±2.
The drum, the boss and the rib are one solid."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Pnt

from ribs import common, fuse, polygon, revolve


def build():
    drum = BRepPrimAPI_MakeCylinder(20, 10).Shape()
    boss = BRepPrimAPI_MakeBox(gp_Pnt(-8, -8, 10), gp_Pnt(8, 8, 40)).Shape()
    body = fuse(drum, boss, "the boss and the drum")
    # The cone the rib's line turns into: from the rim (20, 10) to the axis at z = 10 + 20 / 0.6.
    cone = revolve(polygon([(0, 0, 10), (20, 0, 10), (0, 0, 10 + 100 / 3)]), 360)
    slab = BRepPrimAPI_MakeBox(gp_Pnt(8, -2, 10), gp_Pnt(20, 2, 31)).Shape()
    rib = common(slab, cone, "the slab and the rib's cone")
    return fuse(body, rib, "the rib and the drum")
