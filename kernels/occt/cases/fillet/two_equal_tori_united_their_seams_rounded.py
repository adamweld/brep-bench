"""Two equal tori united, the curves where they cross rounded. Tori of major radius 10 and minor
radius 3 in the xy plane, one about the origin, one moved 10 along x, united; the crossing curves
rounded to radius 1."""
from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.GeomAbs import GeomAbs_SurfaceType
from OCP.TopAbs import TopAbs_EDGE, TopAbs_FACE
from OCP.TopExp import TopExp
from OCP.TopoDS import TopoDS
from OCP.collections import IndexedDataMap_TopoDS_Shape_List_TopoDS_Shape_TopTools_ShapeMapHasher as EdgeFaces
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

MINOR_SECOND = 3
RADIUS = 1


def torus_centre_x(face):
    surface = BRepAdaptor_Surface(face)
    if surface.GetType() != GeomAbs_SurfaceType.GeomAbs_Torus:
        return None
    return round(surface.Torus().Location().X(), 6)


def build():
    first = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 10, 3).Shape()
    second = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(10, 0, 0), gp_Dir(0, 0, 1)), 10, MINOR_SECOND).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not unite the tori")
    solid = fuse.Shape()
    edge_faces = EdgeFaces()
    TopExp.MapShapesAndAncestors_s(solid, TopAbs_EDGE, TopAbs_FACE, edge_faces)
    fillet = BRepFilletAPI_MakeFillet(solid)
    for i in range(1, edge_faces.Extent() + 1):
        centres = {torus_centre_x(TopoDS.Face(f)) for f in edge_faces.FindFromIndex(i)}
        if len(centres) == 2 and None not in centres:  # a face of each torus: a crossing curve
            fillet.Add(RADIUS, TopoDS.Edge(edge_faces.FindKey(i)))
    if fillet.NbContours() == 0:
        raise Refused("no crossing curve found between the tori")
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the crossing curves")
    return fillet.Shape()
