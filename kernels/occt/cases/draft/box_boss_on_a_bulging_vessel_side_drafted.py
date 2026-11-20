"""A box boss on a bulging vessel, its side drafted where it meets the wall. A vessel turned half a
turn from a plane region with a bulging wall (the cubic Bézier through (10, 0), (16, 6), (16, 16),
(10, 22), in the xz plane, turned about the z axis from +x toward +y), and the box with corners
(9, 2, 6) and (20, 8, 16) united with it.

Draft the box's side face y = 8, the part of it that stands outside the vessel, by 4°, with the pull
direction +z and the neutral plane z = 6, the box's bottom: the face leans toward the box's
inside, -y, as it rises. The vessel, the box's other faces and its bottom are not drafted."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused, face_through
from draft import drafted
from vessel import bulging_vessel


def build():
    box = BRepPrimAPI_MakeBox(gp_Pnt(9, 2, 6), gp_Pnt(20, 8, 16)).Shape()
    fuse = BRepAlgoAPI_Fuse(bulging_vessel(), box)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the boss")
    part = fuse.Shape()
    side = face_through(part, (17, 8, 12))  # outside the vessel, whose wall is under 14.5 out
    return drafted(part, [side], 4, neutral=(0, 0, 6))
