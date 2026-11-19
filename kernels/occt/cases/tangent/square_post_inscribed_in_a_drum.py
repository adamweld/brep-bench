"""A square post inscribed in a drum, united. The cylinder of radius 5 on the z axis from z = 0 to
z = 10, its seam at φ = 0, united with the square prism from z = 0 to z = 16 on the square inscribed
in the drum's circle with its corners at φ = 30°, 120°, 210° and 300°: its four upright edges lie
on the drum's wall up to z = 10, and it stands 6 out of the drum's top."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused


def corner(degrees):
    """The square's corner on the drum's circle at φ = `degrees`, in the plane z = 0."""
    return gp_Pnt(5 * math.cos(math.radians(degrees)), 5 * math.sin(math.radians(degrees)), 0)


def build():
    drum = BRepPrimAPI_MakeCylinder(5, 10).Shape()  # its seam at φ = 0
    square = BRepBuilderAPI_MakePolygon(corner(30), corner(120), corner(210), corner(300),
                                        True).Wire()
    post = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(square).Face(), gp_Vec(0, 0, 16)).Shape()
    fuse = BRepAlgoAPI_Fuse(drum, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the drum and the post")
    return fuse.Shape()
