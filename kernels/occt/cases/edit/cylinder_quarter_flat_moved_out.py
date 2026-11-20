"""A quarter of a cylinder, one flat moved out. A quarter of a cylinder: the part of the cylinder of radius 10 about the z axis, from z = 0 to z =
6, whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 90°, so the part
with x ≥ 0 and y ≥ 0. Its faces are the wall, a quarter of a cylinder; a bottom and a top, quarter
disks; and two flat rectangles, 10 by 6, in the half-planes y = 0, x ≥ 0 and x = 0, y ≥ 0, which
meet along the z axis in a convex edge.

Then edit the solid directly: move the flat in the plane y = 0 out by 3 along its outward normal,
−y, to the plane y = −3. The wall runs on round past the old edge to meet it, and the bottom and the
top grow to follow; the other flat is lengthened from y = 0 to y = −3.

So the result is the cylinder cut to x ≥ 0 and y ≥ −3."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import face_through
from edit import moved

def build():
    quarter = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 10, 6, math.pi / 2).Shape()
    flat = face_through(quarter, (5, 0, 3))  # the half-plane y = 0, x >= 0
    return moved(quarter, [(flat, 3)])
