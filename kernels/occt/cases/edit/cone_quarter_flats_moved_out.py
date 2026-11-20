"""A quarter of a cone, both its flats moved out past its axis. A quarter of the cone whose base, of
radius 8, lies in the plane z = 0 centered on the origin and whose apex is (0, 0, 12): the part
whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 90°, so the part
with x ≥ 0 and y ≥ 0. Its faces are the cone; the base, a quarter disk; and two flat triangles
through the axis, in the half-plane y = 0, x ≥ 0 and in the half-plane x = 0, y ≥ 0, which meet
along the axis, from the base's center to the apex, in a convex edge.

Then edit the solid directly: move each flat 3 outward along its normal, the first to the plane y =
−3 and the second to the plane x = −3. The cone and the base run on round past the old edge to meet
both moved flats, and the two flats are extended until they meet each other, along the line x = y =
−3 from the base up to the cone at z = 12(1 − 3√2/8) ≈ 5.64. The apex, where the old edge met the
cone, is left inside the conical face.

So the result is the cone less everything at x < −3 or y < −3."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp

from bench import face_through
from edit import moved


def build():
    cone = BRepPrimAPI_MakeCone(gp.XOY_s(), 8, 0, 12, math.pi / 2).Shape()
    flats = [face_through(cone, (3, 0, 1)), face_through(cone, (0, 3, 1))]
    return moved(cone, [(flat, 3) for flat in flats])
