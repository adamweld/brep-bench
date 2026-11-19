"""A web rib in a shaft's shoulder, its edge turned about the axis. A stepped shaft with one web rib
in the concave corner of its shoulder: a rib whose outer edge is a straight line turned about the
shaft's axis.

- **The shaft:** a cylinder of radius 20 on the z axis from z = 0 to z = 30, and on it a cylinder of
  radius 10 from z = 30 to z = 70. The big cylinder's top, the ring from ρ = 10 to 20 at z = 30, is
  the shoulder.
- **The rib:** its profile is the segment in the half-plane y = 0, x > 0 from (x, z) = (17, 30) on
  the shoulder to (10, 40) on the small cylinder's side. Turned about the z axis, the segment sweeps
  a cone, ρ = 17 − 0.7(z − 30). The rib is the material between the shoulder, the small cylinder's
  side and that cone, within 2 of the plane y = 0 on the +x side: a web 4 thick whose two flat sides
  are the planes y = ±2 and whose outer edge is the cone, not a plane, so that the rib's flat sides
  are bounded there by arcs of hyperbolas.

The rib meets the shoulder from ρ = 10 to ρ = 17 and the small cylinder's side from z = 30 to z =
40. The shaft and the rib are one solid."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepFeat import BRepFeat_MakeRevolutionForm
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.Geom import Geom_Plane
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

HALF_THICKNESS = 2


def build():
    big = BRepPrimAPI_MakeCylinder(20, 30).Shape()
    small = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 30), gp_Dir(0, 0, 1)), 10, 40).Shape()
    shaft = BRepAlgoAPI_Fuse(big, small)
    if not shaft.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the shaft's two cylinders")
    # The rib's open profile: one segment in the plane y = 0, run from the small cylinder's side
    # down to the shoulder (its direction sets the side the rib fills: toward the corner). The
    # rib is HALF_THICKNESS to each side of that plane, its edge the segment turned about z.
    segment = BRepBuilderAPI_MakeEdge(gp_Pnt(10, 0, 40), gp_Pnt(17, 0, 30)).Edge()
    profile = BRepBuilderAPI_MakeWire(segment).Wire()
    plane = Geom_Plane(gp_Pnt(0, 0, 0), gp_Dir(0, 1, 0))
    rib = BRepFeat_MakeRevolutionForm(shaft.Shape(), profile, plane, gp.OZ_s(),
                                      HALF_THICKNESS, HALF_THICKNESS, 1, False)
    rib.Perform()
    if not rib.IsDone():
        raise Refused(f"BRepFeat_MakeRevolutionForm could not make the rib: {rib.CurrentStatusError()}")
    return rib.Shape()
