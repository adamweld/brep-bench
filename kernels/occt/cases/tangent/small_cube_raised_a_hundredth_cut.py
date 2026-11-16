"""A small cube less itself raised a hundredth of its side. A small cube, of side 0.0015, from the
origin to (0.0015, 0.0015, 0.0015), less a copy of itself raised by 0.000015 along z, a hundredth of
its side.

What is left is the slab under the copy: the box from the origin to (0.0015, 0.0015, 0.000015), a
hundredth as thick as it is wide. The four sides of the copy lie in the planes of the cube's own
sides, and its bottom is the slab's top. A kernel that takes the copy's bottom for the cube's own,
by a tolerance that does not shrink with the model, finds nothing left."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused

SIDE = 0.0015
RISE = 0.000015


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(SIDE, SIDE, SIDE)).Shape()
    raised = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, RISE), gp_Pnt(SIDE, SIDE, SIDE + RISE)).Shape()
    result = BRepAlgoAPI_Cut(cube, raised)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the raised cube from the cube")
    return result.Shape()
