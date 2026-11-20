"""A turned post on a dome, its outer side drafted. The hemisphere of radius 20 (z ≥ 0) united with
the rectangle ρ from 3 to 8, z from 0 to 28 in the plane y = 0, turned 120° about the z axis from
+x toward +y; the outer face (ρ = 8) drafted by 4°, pull +z, neutral plane z = 0, into the cone
ρ = 8 − z·tan 4°."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from bench import face_through
from draft import drafted, turned_block, united


def build():
    dome = BRepPrimAPI_MakeSphere(20, 0, math.pi / 2).Shape()
    post = turned_block((0, 0), 3, 8, 0, 28, 120)
    part = united(dome, post)
    side = face_through(part, (4, 6.928203230275509, 24))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
