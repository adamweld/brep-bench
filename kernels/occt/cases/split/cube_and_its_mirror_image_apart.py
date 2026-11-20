"""A cube and its mirror image, apart. The cube [4, 14] x [0, 10] x [0, 10] and its mirror image
across the plane x = 0, [-14, -4] x [0, 10] x [0, 10]: two separate solids, a compound."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopoDS import TopoDS_Builder, TopoDS_Compound
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Trsf


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(4, 0, 0), gp_Pnt(14, 10, 10)).Shape()
    mirror = gp_Trsf()
    mirror.SetMirror(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0)))
    image = BRepBuilderAPI_Transform(cube, mirror, True).Shape()
    compound = TopoDS_Compound()
    builder = TopoDS_Builder()
    builder.MakeCompound(compound)
    builder.Add(compound, cube)
    builder.Add(compound, image)
    return compound
