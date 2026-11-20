"""A quarter of a horn torus, upper half, grown by 1, its edges kept sharp. A horn torus: the solid
swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0, turned about the z axis.
Its tube touches the axis at the origin, where its surface pinches to a point. Take its upper
half, z ≥ 0, turned only through a quarter of a turn, φ from 0° to 90° (φ measured from +x toward
+y). Its faces are the torus, a flat quarter disk of radius 10 in the plane z = 0, and two flat
half disks of radius 5, in the half-plane y = 0, x ≥ 0 and in the half-plane x = 0, y ≥ 0, which
touch each other only at the pinch."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir

from offset import offset
from turned import at, line, turned

TURN = math.pi / 2
DISTANCE = 1


def build():
    # the circle of radius 5 about (5, 0, 0) in the plane y = 0, from (10, 0, 0) over the top to the pinch
    circle = gp_Circ(gp_Ax2(at(5, 0), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0)), 5)
    arch = BRepBuilderAPI_MakeEdge(circle, 0, math.pi).Edge()
    horn = turned([arch, line(at(0, 0), at(10, 0))], TURN, "the horn torus")
    return offset(horn, DISTANCE, GeomAbs_Intersection)
