"""Three quarters of a horn torus, upper half, shrunk by 1. A horn torus: the solid swept by the disk
of radius 5 centered on (5, 0, 0) in the plane y = 0, turned about the z axis. Its tube touches
the axis at the origin, where its surface pinches to a point. Take its upper half, z ≥ 0, turned
through three quarters of a turn, φ from 0° to 270° (φ measured from +x toward +y): all but the
quarter with x ≥ 0 and y ≤ 0. Its faces are the torus, a flat disk of radius 10 less its missing
quarter in the plane z = 0, and two flat half disks of radius 5, in the half-plane y = 0, x ≥ 0
and in the half-plane x = 0, y ≤ 0, which touch each other only at the pinch."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir

from offset import offset
from turned import at, line, turned

TURN = 3 * math.pi / 2
DISTANCE = -1


def build():
    # the circle of radius 5 about (5, 0, 0) in the plane y = 0, from (10, 0, 0) over the top to the pinch
    circle = gp_Circ(gp_Ax2(at(5, 0), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0)), 5)
    arch = BRepBuilderAPI_MakeEdge(circle, 0, math.pi).Edge()
    horn = turned([arch, line(at(0, 0), at(10, 0))], TURN, "the horn torus")
    return offset(horn, DISTANCE)
