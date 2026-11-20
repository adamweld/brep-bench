"""An eccentric tube with six flats and a slanting top. The cylinder of radius 5 on the z axis,
standing on z = 0, cut by the six planes at distance 4.5 from the axis square to the directions 0,
60, ... 300 degrees, below the plane z = 10 + x/4, with a hole of radius 2 bored through along the
line x = 1.5, y = 0."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

RADIUS = 5
APOTHEM = 4.5


def build():
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), RADIUS, 14).Shape()
    corner = APOTHEM / math.cos(math.pi / 6)
    polygon = BRepBuilderAPI_MakePolygon()
    for k in range(6):
        a = math.radians(30 + 60 * k)
        polygon.Add(gp_Pnt(corner * math.cos(a), corner * math.sin(a), -1))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    hexagon = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, 16)).Shape()
    flats = BRepAlgoAPI_Common(cylinder, hexagon)
    if not flats.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the hexagonal prism")
    # The bore runs past both ends.
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(1.5, 0, -2), gp.DZ_s()), 2, 18).Shape()
    bored = BRepAlgoAPI_Cut(flats.Shape(), bore)
    if not bored.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the bore")
    normal = gp_Dir(-0.25, 0, 1)
    along_x = gp_Dir(1, 0, 0.25)
    along_y = normal.Crossed(along_x)
    origin = gp_Pnt(0, 0, 10).Translated(gp_Vec(along_x).Multiplied(-30)).Translated(
        gp_Vec(along_y).Multiplied(-30))
    above = BRepPrimAPI_MakeBox(gp_Ax2(origin, normal, along_x), 60, 60, 30).Shape()
    cut = BRepAlgoAPI_Cut(bored.Shape(), above)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the slanting top")
    return cut.Shape()
