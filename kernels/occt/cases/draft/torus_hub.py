"""A drum standing in a torus, its side drafted. A ring torus about the z axis, the solid swept by
the disk of radius 5 centered on (15, 0, 0) in the plane y = 0 turned a full turn about the z
axis, and a drum, the cylinder of radius 14 about the z axis from z = 0 to z = 12, united. The
drum's side runs down into the tube, and its bottom, in the plane z = 0, shows only inside the
torus's hole, as the disk of radius 10.

Draft the drum's side by 5°, with the pull direction +z and the neutral plane z = 0: the side
becomes the cone through the drum's circle of radius 14 at z = 0, leaning 5° from the z
direction, inward as it rises, its radius 14 − z·tan 5°. It meets the torus at z_a ≈ 4.794,
radius ρ_a ≈ 13.581, and the top shrinks to meet it, to radius 14 − 12·tan 5° ≈ 12.950. The torus
and the drum's top and bottom are not drafted.

So the result is the torus with a frustum of the cone standing in it."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeTorus
from OCP.gp import gp

from bench import Refused, face_through
from draft import drafted


def build():
    ring = BRepPrimAPI_MakeTorus(15, 5).Shape()
    drum = BRepPrimAPI_MakeCylinder(gp.XOY_s(), 14, 12).Shape()
    fuse = BRepAlgoAPI_Fuse(ring, drum)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the drum")
    part = fuse.Shape()
    side = face_through(part, (14, 0, 10))
    return drafted(part, [side], 5, neutral=(0, 0, 0))
