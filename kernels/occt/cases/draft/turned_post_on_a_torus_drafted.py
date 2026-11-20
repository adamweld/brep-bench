"""A turned post on a torus, its outer side drafted. The torus about the z axis, its tube of radius
5 centered on the circle of radius 15 in the plane z = 0, united with the rectangle ρ from 13 to 17,
z from 0 to 12 in the plane y = 0, turned 120° about the z axis from +x toward +y; the outer face
(ρ = 17) drafted by 4°, pull +z, neutral plane z = 0, into the cone ρ = 17 − z·tan 4°."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus

from bench import face_through
from draft import drafted, turned_block, united


def build():
    torus = BRepPrimAPI_MakeTorus(15, 5).Shape()
    post = turned_block((0, 0), 13, 17, 0, 12, 120)
    part = united(torus, post)
    side = face_through(part, (8.5, 14.722431864335457, 10))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
