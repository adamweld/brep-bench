"""Two through-holes in a 40 × 30 × 8 plate, at (15, 15) and (25, 15), widened from radius 3
to radius 6 by offsetting their walls. The walls must trim where the holes merge."""
from OCP.BRepAdaptor import BRepAdaptor_Surface

from bench import face_through
from edit import box, drilled


def build():
    plate = drilled(box((0, 0, 0), (40, 30, 8)), 15, 15, 3, -1, 9)
    plate = drilled(plate, 25, 15, 3, -1, 9)
    walls = [face_through(plate, (x, 18, 4)) for x in (15, 25)]
    # OCCT 8.0.1's SetOffsetOnFace returns a null shape when these walls merge, even with
    # intersection of all faces enabled. Work around it by enlarging each bore with a cut,
    # using its existing wall's axis and radius. The Booleans trim the overlapping walls.
    # This is flagged in workarounds.toml, not claimed as a native face-offset success.
    for wall in walls:
        cylinder = BRepAdaptor_Surface(wall).Cylinder()
        center = cylinder.Location()
        plate = drilled(plate, center.X(), center.Y(), cylinder.Radius() + 3, -1, 9)
    return plate
