"""A cylinder's end face drafted about a diameter, the pull across the axis. A cylinder of radius
10 about the z axis, z = 0 to 20, its top face drafted 8° with the pull direction +x and the
neutral plane x = 0: it turns about the diameter x = 0, z = 20 into z = 20 + x·tan 8°."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from bench import face_through
from draft import drafted


def build():
    cylinder = BRepPrimAPI_MakeCylinder(10, 20).Shape()
    top = face_through(cylinder, (0, 0, 20))
    return drafted(cylinder, [top], 8, neutral=(0, 0, 20), pull=(1, 0, 0))
