"""A NURBS ball less an octant. The ball of radius 5 about the origin bounded by the standard
biquadratic NURBS sphere of geometry/nurbs-sphere (poles on the z axis, seam in the half-plane
y = 0, x ≥ 0), less the box from (0, 0, 0) to (8, 8, 8), whose faces x = 0, y = 0 and z = 0 pass
through the poles and along the seam."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused, other_case


def build():
    ball = other_case("geometry/nurbs-sphere").build()
    box = BRepPrimAPI_MakeBox(8, 8, 8).Shape()
    cut = BRepAlgoAPI_Cut(ball, box)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the box")
    return cut.Shape()
