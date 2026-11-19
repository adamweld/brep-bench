"""A shaft's end centre-drilled: a 60° countersink, a pilot and a 118° point. A shaft, the cylinder of
radius 10 on the z axis from z = 0 to z = 50, with a centre hole drilled into its top end (z = 50)
on the z axis, the kind a lathe centre runs in. Seen from the top, going down, the hole is:

- a countersink, a cone with an included angle of 60° (its side at 30° to the axis), opening into
  the end face in a circle of radius 3.35 and narrowing to radius 1.6 at z = 50 − 1.75√3 ≈ 46.969;
- a pilot of radius 1.6, at its full radius for 4 more, down to z = 46 − 1.75√3 ≈ 42.969;
- the point the drill leaves, a cone with an included angle of 118° (its side at 59° to the axis),
  its base the pilot's circle there and its apex on the axis 1.6 cot 59° ≈ 0.961 below."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from bench import Refused
from hole import cutter

MOUTH, PILOT, PILOT_LENGTH = 3.35, 1.6, 4
SINK = (MOUTH - PILOT) / math.tan(math.radians(30))  # the 60° countersink's depth
POINT = PILOT / math.tan(math.radians(59))  # the 118° point's
# The centre drill's half section from the end face down, the countersink run on 1 above it.
SECTION = [(0, -1), (MOUTH + math.tan(math.radians(30)), -1), (PILOT, SINK),
           (PILOT, SINK + PILOT_LENGTH), (0, SINK + PILOT_LENGTH + POINT)]


def build():
    shaft = BRepPrimAPI_MakeCylinder(10, 50).Shape()
    cut = BRepAlgoAPI_Cut(shaft, cutter((0, 0, 50), (0, 0, -1), SECTION))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the centre hole")
    return cut.Shape()
