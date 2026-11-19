"""A gusset rib from a plate up to a square boss's side. A plate with a square boss standing on it,
braced by a gusset: a rib made from one straight line running from the plate's top up to the boss's
side.

- **The plate:** the box from (0, −20, 0) to (60, 20, 8).
- **The boss:** the box from (0, −10, 8) to (20, 10, 38), standing on the plate's top at its end;
  its face x = 0 lies in the plane of the plate's end face x = 0, the two making one face.
- **The rib:** the open profile is the segment in the plane y = 0 from (50, 0, 8) on the plate's top
  to (20, 0, 30) on the boss's side x = 20. The rib fills the space between that segment, the
  plate's top and the boss's side, the right triangle (20, 8), (50, 8), (20, 30) in (x, z), and is 4
  thick, 2 to each side of the plane y = 0.

The plate, the boss and the rib are one solid."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepFeat import BRepFeat_MakeLinearForm
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.Geom import Geom_Plane
from OCP.gp import gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

HALF_THICKNESS = 2


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(0, -20, 0), gp_Pnt(60, 20, 8)).Shape()
    boss = BRepPrimAPI_MakeBox(gp_Pnt(0, -10, 8), gp_Pnt(20, 10, 38)).Shape()
    body = BRepAlgoAPI_Fuse(plate, boss)
    if not body.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the boss to the plate")
    # The rib's open profile: one segment in the plane y = 0, run from the boss's side down to the
    # plate's top (its direction sets the side the rib fills: toward the corner), thickened
    # HALF_THICKNESS to each side of that plane.
    segment = BRepBuilderAPI_MakeEdge(gp_Pnt(20, 0, 30), gp_Pnt(50, 0, 8)).Edge()
    profile = BRepBuilderAPI_MakeWire(segment).Wire()
    plane = Geom_Plane(gp_Pnt(0, 0, 0), gp_Dir(0, 1, 0))
    rib = BRepFeat_MakeLinearForm(body.Shape(), profile, plane, gp_Vec(0, HALF_THICKNESS, 0),
                                  gp_Vec(0, -HALF_THICKNESS, 0), 1, True)
    rib.Perform()
    if not rib.IsDone():
        raise Refused(f"BRepFeat_MakeLinearForm could not make the rib: {rib.CurrentStatusError()}")
    return rib.Shape()
