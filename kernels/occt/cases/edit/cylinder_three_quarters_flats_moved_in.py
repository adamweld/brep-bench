"""Three quarters of a cylinder, both flats moved in. Three quarters of a cylinder: the part of the cylinder of radius 10 about the z axis, from z = 0 to
z = 6, whose angle about the z axis, φ, measured from +x toward +y, lies between 90° and 360°, so
the disk less its quadrant x > 0, y > 0. Its faces are the wall, three quarters of a cylinder; a
bottom and a top, three quarters of disks; and two flat rectangles, 10 by 6, in the half-planes x =
0, y ≥ 0 and y = 0, x ≥ 0, which meet along the z axis in a concave edge.

Then edit the solid directly: move each flat 2 inward along its normal, into the solid, the first to
the plane x = −2 and the second to the plane y = −2. The wall is trimmed back to meet them, and the
flats are extended until they meet each other, along the line x = y = −2.

So the result is the cylinder with everything at x ≥ −2 and y ≥ −2 taken away. Each flat runs from
the line x = y = −2 to the wall, 4√6 + 2 wide."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import face_through
from edit import moved

def build():
    # from φ = 90° (the +y direction) through 360°
    three = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(0, 1, 0)),
                                     10, 6, 3 * math.pi / 2).Shape()
    flats = [face_through(three, (0, 5, 3)), face_through(three, (5, 0, 3))]
    return moved(three, [(flat, -2) for flat in flats])
