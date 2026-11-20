"""A block standing out of a cylinder's side, its flanks drafted. A cylinder of radius 10 about the
z axis, z = 0 to 20, and a block 0 ≤ x ≤ 16, −3 ≤ y ≤ 3, 5 ≤ z ≤ 15, united. The block's two flanks
y = ±3 are drafted 5°, pull +z, neutral plane z = 5, so they lean in as they rise."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Pnt

from bench import Refused, face_through
from draft import drafted


def build():
    base = BRepPrimAPI_MakeCylinder(10, 20).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, -3, 5), gp_Pnt(16, 3, 15)).Shape()
    fuse = BRepAlgoAPI_Fuse(base, block)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the block")
    part = fuse.Shape()
    flanks = [face_through(part, (13, 3, 10)), face_through(part, (13, -3, 10))]
    return drafted(part, flanks, 5, neutral=(0, 0, 5))
