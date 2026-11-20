"""The band of the sphere of radius 5 about the origin that lies between the equator, z = 0, and the circle at height z = 3 (where the sphere's radius across is 4), in the upper half, all the way round. Extrude it straight up the z axis by 2: the solid of everything the band passes through as it is carried up 2 without turning.

The solid lies between the band and its copy 2 higher, over the ring 4 \le \rho \le 5; its other faces are the cylinders \rho = 4 (from z = 3 to 5) and \rho = 5 (from z = 0 to 2), each 2 high.
"""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism, BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax2, gp_Circ, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused


def build():
    # The meridian from the equator (5, 0, 0) up to (4, 0, 3), in the xz plane. The circle's frame
    # has normal -y and x direction +x, so its angle runs from +x toward +z.
    meridian = BRepBuilderAPI_MakeEdge(
        gp_Circ(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0)), 5),
        0, math.asin(3 / 5)).Edge()
    # Turned a whole turn about the z axis, the edge makes the band, a face.
    band = BRepPrimAPI_MakeRevol(meridian, gp.OZ_s()).Shape()
    # Extruded straight up, the face makes a solid.
    prism = BRepPrimAPI_MakePrism(band, gp_Vec(0, 0, 2))
    if not prism.IsDone():
        raise Refused("BRepPrimAPI_MakePrism could not extrude the band")
    return prism.Shape()
