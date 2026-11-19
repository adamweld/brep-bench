"""A tapered hole through a plate, 1 in 16 on the diameter. A plate, the box from the origin to (40,
40, 16), with a tapered hole right through it, its axis the vertical line through (20, 20). The
hole is a cone about that axis: its radius is 5 at the top face (z = 16) and 4.5 at the bottom face
(z = 0), narrowing evenly between, a taper of 1 in 16 on the diameter (as a pipe thread's is). Its
wall is the one cone from the top face to the bottom."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused
from hole import cutter

# The taper's half section from the top face down: radius 5 at the face, narrowing by 1/32 for
# each unit of depth, run on 1 past each face.
SECTION = [(0, -1), (5 + 1 / 32, -1), (4.5 - 1 / 32, 17), (0, 17)]


def build():
    plate = BRepPrimAPI_MakeBox(40, 40, 16).Shape()
    cut = BRepAlgoAPI_Cut(plate, cutter((20, 20, 16), (0, 0, -1), SECTION))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the tapered hole")
    return cut.Shape()
