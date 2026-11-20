"""A post standing by a rounded rim, its foot rounded across the rim's round. A short wide cylinder with its rim already rounded, and a post standing near the rim, the post's foot
then rounded where it meets the top and the round together.

- **The cylinder:** radius 10 on the z axis from z = 0 to z = 10, the circular edge around its top
  face rounded to radius 2: the round is a quarter of a torus, centered on the circle of radius 8 at
  z = 8, tangent to the top at ρ = 8 and to the side at z = 8.
- **The post:** the cylinder of radius 3 whose axis is the vertical line through (6.5, 0), from z = 0
  to z = 20, joined to the first. Its wall reaches ρ = 9.5 at its far side, past the top's edge at
  ρ = 8, so there it stands out of the round, not the flat top: the post's wall meets the round where
  it is 0.68 below the top.

Round the foot of the post to radius 0.5: the whole edge where the post's wall meets the flat top and
the round, a closed curve running over the round's tangent line at ρ = 8. It is a concave round, a
ball of radius 0.5 rolling over the top and the round and touching the post's wall, which adds
material."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from blend import edges_where, rounded
from chamfer import edge_through


def build():
    base = BRepPrimAPI_MakeCylinder(10, 10).Shape()
    base = rounded(base, 2, [edge_through(base, (0, 10, 10))], "the cylinder's rim")
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(6.5, 0, 0), gp_Dir(0, 0, 1)), 3, 20).Shape()
    fuse = BRepAlgoAPI_Fuse(base, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the post to the cylinder")
    solid = fuse.Shape()
    # The foot: the edges on the post's wall just below and at the height of the top (z = 10).
    foot = edges_where(solid, lambda p: abs(((p.X() - 6.5) ** 2 + p.Y() ** 2) ** 0.5 - 3) < 1e-6
                       and 9 < p.Z() < 10 + 1e-6)
    return rounded(solid, 0.5, foot, "the post's foot")
