r"""A part-turned boss on a plate, its outer wall drafted. A square plate with a part-turned boss on it, a tall arc-shaped wall with a flat end at each end, the boss's outer wall drafted.

The plate is the box −12 ≤ x, y ≤ 12, 0 ≤ z ≤ 4. On it stands the boss: the rectangle 2 ≤ r ≤ 5, 4 ≤ z ≤ 12 of the half-plane φ = 0 (r the distance from the z axis, φ the angle about it from +x toward +y), turned about the z axis from φ = 0 to φ = 240°, and united with the plate. It is a wall 3 thick and 8 tall that runs two thirds of the way round a hollow of radius 2: an outer cylinder of radius 5, an inner cylinder of radius 2, a flat top at z = 12, and two flat end faces in the planes φ = 0 and φ = 240°, each through the z axis, standing on the plate's top.

Draft only the boss's outer cylindrical wall, 5°, with the pull direction +z and the neutral plane z = 4, the plate's top, where the boss stands: the wall becomes a cone through the circle of radius 5 at z = 4, leaning 5° from the vertical, inward as it rises, so that its radius is 5 − (z − 4)·tan 5°. The inner wall, the top, the two end faces and the plate stay as they are, except that the top and the end faces are cut back to meet the cone: the top is the sector 2 ≤ r ≤ 5 − 8·tan 5° (about 4.3) and each end face the quadrilateral 2 ≤ r ≤ 5 − (z − 4)·tan 5° of its plane.

**As a set:**

$$\{|x|, |y| \le 12,\ 0 \le z \le 4\}\ \cup\ \{\,4 \le z \le 12,\ 2 \le r \le 5 - (z - 4)\tan 5^\circ,\ 0 \le \varphi \le 240^\circ\,\}$$"""

import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt

from bench import Refused, face_through
from draft import drafted

HALF, THICKNESS = 12, 4
R_IN, R_OUT, HEIGHT = 2, 5, 8
TURN = 240  # degrees
DRAFT = 5  # degrees, inward as the wall rises


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-HALF, -HALF, 0), gp_Pnt(HALF, HALF, THICKNESS)).Shape()
    z0, z1 = THICKNESS, THICKNESS + HEIGHT
    profile = BRepBuilderAPI_MakePolygon()
    for r, z in [(R_IN, z0), (R_OUT, z0), (R_OUT, z1), (R_IN, z1)]:
        profile.Add(gp_Pnt(r, 0, z))
    profile.Close()
    face = BRepBuilderAPI_MakeFace(profile.Wire()).Face()
    boss = BRepPrimAPI_MakeRevol(face, gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)),
                                 math.radians(TURN)).Shape()
    fuse = BRepAlgoAPI_Fuse(plate, boss)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the boss")
    part = fuse.Shape()
    phi = math.radians(TURN / 2)
    side = face_through(part, (R_OUT * math.cos(phi), R_OUT * math.sin(phi), z0 + HEIGHT / 2))
    return drafted(part, [side], DRAFT, neutral=(0, 0, THICKNESS))
