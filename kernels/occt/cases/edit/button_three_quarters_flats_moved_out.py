"""Three quarters of a domed button, both its flats moved out. The button: the solid of revolution about the z axis of the region of the half-plane ρ ≥ 0 (ρ the
distance from the axis) bounded by the axis from z = 0 to z = 5, the base, ρ from 0 to 6 at z = 0,
the straight line from (ρ, z) = (6, 0) to (4, 2), the flare, and the quarter of the ellipse (ρ/4)²
+ ((z − 2)/3)² = 1 from (4, 2) to (0, 5), the dome. The flare meets the dome at a concave edge,
the circle ρ = 4 at z = 2, and the dome rises from it vertically. Take three quarters of it: the
part whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 270°. Its
faces are the base, the flare and the dome, three quarters of each, and two flat faces through the
axis, each the half-section above, in the half-planes y = 0, x ≥ 0 (φ = 0°) and x = 0, y ≤ 0 (φ =
270°), which meet along the axis in a concave edge.

Then edit the solid directly: move the flat at φ = 0° by 1 outward along its normal, −y, to the
plane y = −1, and the flat at φ = 270° by 1 outward along its normal, +x, to the plane x = 1. The
faces beside them follow: the base, the flare and the dome run on round into the quarter that was
missing, as far as the moved flats, which meet along the line x = 1, y = −1, a concave edge.

So the result is the whole button less its part with x > 1 and y < −1."""
import math

from button import button

from bench import face_through
from edit import moved

DISTANCE = 1


def build():
    button3 = button(3 * math.pi / 2)
    first = face_through(button3, (3, 0, 1))   # the half-section at φ = 0°, its outward normal −y
    last = face_through(button3, (0, -3, 1))   # the half-section at φ = 270°, its outward normal +x
    return moved(button3, [(first, DISTANCE), (last, DISTANCE)])
