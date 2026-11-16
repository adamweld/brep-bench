"""A turned post's rounded shoulder deleted, its cone made turned a radian about the axis: the cylinder and the cone meet again."""
import math
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt
from blend import edges_where, rounded
from edit import deleted, on_circle
from sweep import turn
from OCP.gp import gp_Ax1
def near(solid, p):
    from OCP.BRepExtrema import BRepExtrema_DistShapeShape
    from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeVertex
    from OCP.TopAbs import TopAbs_FACE
    from OCP.TopExp import TopExp_Explorer
    from OCP.TopoDS import TopoDS
    v = BRepBuilderAPI_MakeVertex(gp_Pnt(*p)).Vertex(); best = None
    e = TopExp_Explorer(solid, TopAbs_FACE)
    while e.More():
        f = TopoDS.Face(e.Current()); d = BRepExtrema_DistShapeShape(v, f); d.Perform()
        if best is None or d.Value() < best[0]: best = (d.Value(), f)
        e.Next()
    return best[1]
def build():
    b = BRepAlgoAPI_Fuse(BRepPrimAPI_MakeCylinder(5, 6).Shape(), turn(BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 6), gp_Dir(0, 0, 1)), 5, 2, 4).Shape(), gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 57.29577951308232)).Shape()
    r = rounded(b, 1, edges_where(b, on_circle((0, 0, 6), 5)), 'edge')
    faces = []
    for k in range(8):
        t = 0.1 + k * math.pi / 4
        f = near(r, (4.948683298050514 * math.cos(t), 4.948683298050514 * math.sin(t), 5.982894432683505))
        if not any(f.IsSame(g) for g in faces):
            faces.append(f)
    return deleted(r, faces)
