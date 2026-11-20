"""An elliptic cylinder shelled with its top open: an elliptic cup. The ellipse x²/25 + y²/9 ≤ 1 in
the plane z = 0 extruded straight up to z = 10 (geometry/elliptic-cylinder), shelled inward to a
wall 1 thick, open at its top face, z = 10."""
from bench import other_case
from shell import face_through, shell

HEIGHT = 10
THICKNESS = 1


def build():
    cylinder = other_case("geometry/elliptic-cylinder").build()
    return shell(cylinder, [face_through(cylinder, (0, 0, HEIGHT))], THICKNESS)
