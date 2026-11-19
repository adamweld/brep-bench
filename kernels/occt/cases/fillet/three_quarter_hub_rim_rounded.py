"""Three quarters of a turned hub, the rim between its cylinder and its cone rounded. A hub turned
three quarters of a turn: the polygon through (ρ, z) = (0, 0), (20, 0), (20, 6), (12, 6), (12,
16), (8, 20), (0, 20), drawn in the half-plane y = 0, x ≥ 0 (where ρ = x), turned about the z
axis from φ = 0° to φ = 270°, right-handed. So it is a flange of radius 20, 6 thick; on it a
cylinder of radius 12 up to z = 16; and on that a cone at 45° up to a flat top of radius 8 at z =
20; all cut by the half-planes φ = 0° and φ = 270°, which meet along the z axis in a concave
edge.

Round the convex edge where the cylinder meets the cone, three quarters of the circle of radius
12 at z = 16, to radius 4. The round is the torus swept by the circle of radius 4 tangent to
both, whose center in the section is (8, 16 − t), t = 4 tan 22.5° ≈ 1.657: in the section, an arc
of 45° from (12, 16 − t) on the cylinder to (12 − t/√2, 16 + t/√2) on the cone. It runs the
edge's whole length and ends in the two flat faces at φ = 0° and φ = 270°, whose corners it
rounds the same way."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt

from blend import edges_where, rounded

PROFILE = [(0, 0), (20, 0), (20, 6), (12, 6), (12, 16), (8, 20), (0, 20)]  # (ρ, z)
RADIUS = 4


def build():
    outline = BRepBuilderAPI_MakePolygon()
    for rho, z in PROFILE:
        outline.Add(gp_Pnt(rho, 0, z))
    outline.Close()
    hub = BRepPrimAPI_MakeRevol(BRepBuilderAPI_MakeFace(outline.Wire()).Face(), gp.OZ_s(),
                                3 * math.pi / 2).Shape()
    rim = edges_where(hub, lambda p: abs(math.hypot(p.X(), p.Y()) - 12) < 1e-7
                      and abs(p.Z() - 16) < 1e-7)
    return rounded(hub, RADIUS, rim, "the rim where the cylinder meets the cone")
