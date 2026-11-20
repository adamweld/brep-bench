"""A plate's three edges at a corner rounded with unequal radii, creased where they meet. The box
from the origin to (20, 12, 4); at the corner (20, 0, 4) the edge along x (y = 0, z = 4) is rounded
to radius 1, the edge along y (x = 20, z = 4) to radius 2, and the upright edge (x = 20, y = 0) to
radius 1.5, each running the whole length of its edge."""
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused
from blend import edges_where

TOLERANCE = 1e-7


def build():
    plate = BRepPrimAPI_MakeBox(20, 12, 4).Shape()
    # Rounded one after another, OCCT's second round already takes the upright edge with it, so all
    # three go into one fillet, each with its own radius.
    fillet = BRepFilletAPI_MakeFillet(plate)
    for radius, test in (
            (1, lambda p: abs(p.Y()) < TOLERANCE and abs(p.Z() - 4) < TOLERANCE),
            (2, lambda p: abs(p.X() - 20) < TOLERANCE and abs(p.Z() - 4) < TOLERANCE),
            (1.5, lambda p: abs(p.X() - 20) < TOLERANCE and abs(p.Y()) < TOLERANCE)):
        for edge in edges_where(plate, test):
            fillet.Add(radius, edge)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the three edges")
    return fillet.Shape()
