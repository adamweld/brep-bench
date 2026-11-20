"""A quarter of a cylinder, both flats moved in. A quarter of a cylinder: the part of the cylinder of radius 10 about the z axis, from z = 0 to z =
6, whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 90°, so the part
with x ≥ 0 and y ≥ 0. Its faces are the wall, a quarter of a cylinder; a bottom and a top, quarter
disks; and two flat rectangles, 10 by 6, in the half-planes y = 0, x ≥ 0 and x = 0, y ≥ 0, which
meet along the z axis in a convex edge.

Then edit the solid directly: move each flat 2 inward along its normal, the first to the plane y = 2
and the second to the plane x = 2. The wall is trimmed back to meet them, and the flats are trimmed
against each other, meeting along the line x = y = 2.

So the result is the cylinder cut to x ≥ 2 and y ≥ 2. Each flat runs from the line x = y = 2 to the
wall, 4√6 − 2 wide."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import face_through
from edit import moved

def build():
    quarter = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 10, 6, math.pi / 2).Shape()
    flats = [face_through(quarter, (5, 0, 3)), face_through(quarter, (0, 5, 3))]
    return moved(quarter, [(flat, -2) for flat in flats])
