"""A web rib with an L-shaped edge turned about a three-stage shaft's cone, the shaft's ends rounded. A three-stage shaft with a web rib in the flank of its cone.

- **The shaft:** the solid turned about the z axis from the half-section with corners (ρ, z) = (0, 0),
  (20, 0), (20, 30), (10, 50), (10, 70), (0, 70): a cylinder of radius 20 from z = 0 to 30, a cone
  frustum narrowing from radius 20 at z = 30 to radius 10 at z = 50, and a cylinder of radius 10 up
  to z = 70. Its two end rims, the circle of radius 20 at z = 0 and the circle of radius 10 at
  z = 70, are each rounded to radius 2.
- **The rib:** its edge is an L turned about the axis. The profile is two segments in the half-plane
  y = 0, x > 0: from (x, z) = (10, 56) on the small cylinder's side to (14, 56), and from there down
  to (14, 42) on the cone. Turned about the z axis the first sweeps a flat ring, the plane z = 56 from
  ρ = 10 to 14, and the second a cylinder of radius 14. The rib is the material between the cone, the
  small cylinder's side, that ring and that cylinder, within 2 of the plane y = 0 on the +x side: a
  web 4 thick whose two flat sides are the planes y = ±2. As a set it is
  { |y| ≤ 2, x > 0, 10 ≤ ρ ≤ 14, 70 − 2ρ ≤ z ≤ 56 }.

The rib meets the cone from ρ = 10 to ρ = 14 and the small cylinder's side from z = 50 to z = 56. It
is clear of the rounded rims. The shaft and the rib are one solid."""
from OCP.BRepFeat import BRepFeat_MakeRevolutionForm
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.Geom import Geom_Plane
from OCP.gp import gp, gp_Dir, gp_Pnt

from bench import Refused
from blend import rounded
from chamfer import edge_through
from vround import turned

HALF_THICKNESS = 2


def build():
    shaft = turned([(0, 0), (20, 0), (20, 30), (10, 50), (10, 70), (0, 70)])
    # The two end rims, each rounded to 2.
    rims = [edge_through(shaft, (0, 20, 0)), edge_through(shaft, (0, 10, 70))]
    shaft = rounded(shaft, 2, rims, "the shaft's two end rims")
    # The rib's open profile in the plane y = 0, from the small cylinder's side out and down to
    # the cone. The rib is HALF_THICKNESS to each side of that plane, its edge the L turned.
    across = BRepBuilderAPI_MakeEdge(gp_Pnt(10, 0, 56), gp_Pnt(14, 0, 56)).Edge()
    down = BRepBuilderAPI_MakeEdge(gp_Pnt(14, 0, 56), gp_Pnt(14, 0, 42)).Edge()
    wire = BRepBuilderAPI_MakeWire(across)
    wire.Add(down)
    plane = Geom_Plane(gp_Pnt(0, 0, 0), gp_Dir(0, 1, 0))
    rib = BRepFeat_MakeRevolutionForm(shaft, wire.Wire(), plane, gp.OZ_s(),
                                      HALF_THICKNESS, HALF_THICKNESS, 1, False)
    rib.Perform()
    if not rib.IsDone():
        raise Refused(f"BRepFeat_MakeRevolutionForm could not make the rib: {rib.CurrentStatusError()}")
    return rib.Shape()
