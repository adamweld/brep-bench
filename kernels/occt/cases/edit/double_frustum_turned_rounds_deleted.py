"""Two cone frustums base to base, the upper made turned two radians about the axis, their rounded ridge deleted: the two cones meet again."""
import math
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
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
    b = BRepAlgoAPI_Fuse(BRepPrimAPI_MakeCone(3, 5, 4).Shape(), turn(BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 4), gp_Dir(0, 0, 1)), 5, 2, 4).Shape(), gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 114.59155902616465)).Shape()
    r = rounded(b, 1, edges_where(b, on_circle((0, 0, 4), 5)), 'edge')
    faces = []
    for k in range(8):
        t = 0.1 + k * math.pi / 4
        f = near(r, (4.825138920703175 * math.cos(t), 4.825138920703175 * math.sin(t), 3.984232786315833))
        if not any(f.IsSame(g) for g in faces):
            faces.append(f)
    return deleted(r, faces)
