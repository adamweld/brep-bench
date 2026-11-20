"""A block standing out of a cone frustum's side, its flanks drafted. A frustum about the z axis,
radius 10 − z/5, z = 0 to 20, and a block 0 ≤ x ≤ 16, −3 ≤ y ≤ 3, 5 ≤ z ≤ 15, united. The flanks
y = ±3 are drafted 5°, pull +z, neutral plane z = 5."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp_Pnt

from bench import Refused, face_through
from draft import drafted


def build():
    base = BRepPrimAPI_MakeCone(10, 6, 20).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, -3, 5), gp_Pnt(16, 3, 15)).Shape()
    fuse = BRepAlgoAPI_Fuse(base, block)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the block")
    part = fuse.Shape()
    flanks = [face_through(part, (13, 3, 10)), face_through(part, (13, -3, 10))]
    return drafted(part, flanks, 5, neutral=(0, 0, 5))
