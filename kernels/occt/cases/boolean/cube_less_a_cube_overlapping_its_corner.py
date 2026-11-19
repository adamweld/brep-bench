"""A cube less a cube overlapping its corner. A cube of side 10 from the origin to (10, 10, 10), and a cube of side 6 from (6, 6, 6) to (12, 12, 12). The small cube's corner (6, 6, 6) lies inside the large one and the large cube's corner (10, 10, 10) inside the small one, so the two overlap in the cube from (6, 6, 6) to (10, 10, 10), of side 4. Cut the small cube from the large one."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def box(x0, y0, z0, x1, y1, z1):
    return BRepPrimAPI_MakeBox(gp_Pnt(x0, y0, z0), gp_Pnt(x1, y1, z1)).Shape()


def run(op, name):
    if not op.IsDone():
        raise Refused(f"{name} declined")
    return op.Shape()


def build():
    big = box(0, 0, 0, 10, 10, 10)
    small = box(6, 6, 6, 12, 12, 12)
    return run(BRepAlgoAPI_Cut(big, small), "BRepAlgoAPI_Cut")
