"""A quarter of a cylinder, its ends moved out. A quarter of a cylinder: the part of the cylinder of radius 10 about the z axis, from z = 0 to z =
6, whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 90°, so the part
with x ≥ 0 and y ≥ 0. Its faces are the wall, a quarter of a cylinder; a bottom and a top, quarter
disks; and two flat rectangles, 10 by 6, in the half-planes y = 0, x ≥ 0 and x = 0, y ≥ 0, which
meet along the z axis in a convex edge.

Then edit the solid directly: move the bottom down by 2 and the top up by 3, each along its outward
normal, to the planes z = −2 and z = 9. The wall and the two flats run on to meet them.

So the result is the same quarter of a cylinder, from z = −2 to z = 9."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import face_through
from edit import moved

def build():
    quarter = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 10, 6, math.pi / 2).Shape()
    bottom = face_through(quarter, (1, 1, 0))
    top = face_through(quarter, (1, 1, 6))
    return moved(quarter, [(bottom, 2), (top, 3)])
