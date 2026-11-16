"""A small cube united with itself raised a hundredth of its side. A small cube, of side 0.0015, from
the origin to (0.0015, 0.0015, 0.0015), united with a copy of itself raised by 0.000015 along z: a
hundredth of its side.

The union is the box from the origin to (0.0015, 0.0015, 0.001515). The four sides of the copy lie in
the planes of the cube's own sides; its top and bottom stand 0.000015 above the cube's. That is a
small distance, but it is 1% of the solid: a kernel that takes the copy's top for the cube's own, by
a tolerance that does not shrink with the model, returns the cube and is 1% out."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused

SIDE = 0.0015
RISE = 0.000015


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(SIDE, SIDE, SIDE)).Shape()
    raised = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, RISE), gp_Pnt(SIDE, SIDE, SIDE + RISE)).Shape()
    result = BRepAlgoAPI_Fuse(cube, raised)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two cubes")
    return result.Shape()
