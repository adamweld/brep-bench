#!/usr/bin/env python3
"""Check that the two measurers agree: harness/mass.py and its fast twin, harness/mass (Rust),
over every STEP file under the folders (or the files) named.

    cargo build --release --manifest-path harness/mass/Cargo.toml
    python3 harness/mass/compare.py results/meshes/occt            # a kernel's files
    python3 harness/mass/compare.py FILE.step --timeout 600        # one file, however slow

Each file is measured by both, each in a process of its own: Python's `--jobs` at once (eight by
default), each in `--timeout` seconds (30, the harness's own limit; a file Python does not measure
in time is only reported, with the Rust tool's answer and time), then the Rust tool's one at a time,
so that its times are its own. The answers agree where they have the same keys, the same words
(an error's text, the counts of solids and faces) and numbers within `--tol` (1e-9) of each other,
as a fraction of the solid's own measure: a volume or an area of itself, a centroid's coordinate
of the solid's size (the square root of its area, or the centroid's own distance from the origin
if that is more), an inertia's term of the largest moment. It prints the files that disagree, the
largest difference found, and the times; and exits with failure if any disagree.

`--save FILE` keeps every answer and time as JSON; `--python FILE` takes Python's answers and
times from such a file where it has them, instead of running it again. Standard library only.
"""
import argparse
import concurrent.futures
import json
import math
import os
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
PYTHON = [sys.executable, "-I", os.path.join(os.path.dirname(HERE), "mass.py")]
RUST = [os.path.join(HERE, "target", "release", "mass")]


def run(command, path, timeout):
    """(answer or None, seconds, what it said if it gave no answer); the answer None and the time
    None where it ran out of time."""
    start = time.perf_counter()
    try:
        p = subprocess.run(command + [path], capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return None, None, "out of time"
    took = time.perf_counter() - start
    lines = p.stdout.strip().splitlines()
    try:
        return json.loads(lines[-1]), took, ""
    except (IndexError, ValueError):
        return None, took, (p.stderr.strip().splitlines() or ["no answer"])[-1]


def differences(a, b, tol, at=""):
    """Where two answers differ: [(where, how)], and the largest relative difference of their
    numbers."""
    out, worst = [], 0.0

    def number(x, y, scale, where):
        nonlocal worst
        if isinstance(x, bool) or isinstance(y, bool) or not isinstance(x, (int, float)) \
                or not isinstance(y, (int, float)):
            if x != y:
                out.append((where, f"{x!r} and {y!r}"))
            return
        if math.isnan(x) or math.isnan(y) or math.isinf(x) or math.isinf(y):
            if repr(float(x)) != repr(float(y)):
                out.append((where, f"{x!r} and {y!r}"))
            return
        d = abs(x - y) / scale if scale else (0.0 if x == y else math.inf)
        worst = max(worst, d)
        if d > tol:
            out.append((where, f"{x!r} and {y!r}: {d:.3g} of {scale:.6g}"))

    if set(a) != set(b):
        return [(at or "the answer", f"keys {sorted(a)} and {sorted(b)}")], math.inf
    for key in a:
        x, y = a[key], b[key]
        where = at + key
        if key == "by_surface":
            more, w = differences(x, y, tol, "by_surface.")
            out += more
            worst = max(worst, w)
        elif key in ("volume", "area"):
            number(x, y, max(abs(x), abs(y)), where)
        elif key == "centroid":
            size = max(math.sqrt(abs(a.get("area", 0.0))), *(abs(c) for c in x + y))
            for i in range(3):
                number(x[i], y[i], size, f"{where}[{i}]")
        elif key == "inertia":
            size = max(abs(m[i][i]) for m in (x, y) for i in range(3))
            for i in range(3):
                for j in range(3):
                    number(x[i][j], y[i][j], size, f"{where}[{i}][{j}]")
        elif x != y:
            out.append((where, f"{x!r} and {y!r}"))
    return out, worst


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("paths", nargs="+", help="STEP files, or folders searched for *.step")
    ap.add_argument("--jobs", type=int, default=8, help="how many files Python measures at once")
    ap.add_argument("--timeout", type=float, default=30.0,
                    help="how long either may take over a file, in seconds")
    ap.add_argument("--tol", type=float, default=1e-9, help="how far two numbers may differ")
    ap.add_argument("--save", help="write every answer and time to this file, as JSON")
    ap.add_argument("--python", help="take Python's answers from this file (one --save wrote)")
    args = ap.parse_args()
    if not os.path.isfile(RUST[0]):
        sys.exit(f"{RUST[0]} is not built: cargo build --release --manifest-path "
                 f"{os.path.join(HERE, 'Cargo.toml')}")
    files = []
    for p in args.paths:
        if os.path.isdir(p):
            for d, _, names in os.walk(p):
                files += [os.path.join(d, n) for n in names if n.lower().endswith((".step", ".stp"))]
        else:
            files.append(p)
    files.sort()
    kept = {}
    if args.python:
        with open(args.python) as f:
            kept = {r["file"]: r["python"] for r in json.load(f) if "python" in r}
    todo = [p for p in files if p not in kept]
    with concurrent.futures.ThreadPoolExecutor(max(1, args.jobs)) as pool:
        python = dict(zip(todo, pool.map(lambda p: run(PYTHON, p, args.timeout), todo)))
    python.update({p: tuple(kept[p]) for p in files if p in kept})
    rust = {p: run(RUST, p, args.timeout) for p in files}

    rows, wrong, late, worst = [], [], [], (0.0, None)
    for p in files:
        (pa, pt, psaid), (ra, rt, rsaid) = python[p], rust[p]
        rows.append({"file": p, "python": [pa, pt, psaid], "rust": [ra, rt, rsaid]})
        if pt is None:
            late.append(p)
            continue
        if pa is None or ra is None:
            # no answer from one of them: both must have said nothing, and failed alike
            if pa is not None or ra is not None:
                wrong.append((p, [("the answer", f"Python {pa or psaid!r}, Rust {ra or rsaid!r}")]))
            continue
        diffs, w = differences(pa, ra, args.tol)
        if w > worst[0] and math.isfinite(w):
            worst = (w, p)
        if diffs:
            wrong.append((p, diffs))
    if args.save:
        with open(args.save, "w") as f:
            json.dump(rows, f, indent=1)

    both = [p for p in files if python[p][1] is not None]
    print(f"{len(files)} files; {len(both)} measured by both, {len(late)} not by Python in "
          f"{args.timeout:g} s")
    kinds = {}
    for p in both:
        a = python[p][0]
        kind = "no answer" if a is None else "invalid" if "invalid" in a else \
            "error" if "error" in a else "measured"
        kinds[kind] = kinds.get(kind, 0) + 1
    print("Python's answers: " + ", ".join(f"{n} {k}" for k, n in sorted(kinds.items())))
    print(f"disagreeing: {len(wrong)}")
    for p, diffs in wrong:
        print(f"  {p}")
        for where, how in diffs:
            print(f"    {where}: {how}")
    print(f"largest difference of a number: {worst[0]:.3g}" + (f" ({worst[1]})" if worst[1] else ""))
    pt = sum(python[p][1] for p in both)
    rt = sum(rust[p][1] or args.timeout for p in both)
    print(f"time over the files both measured: Python {pt:.1f} s, Rust {rt:.2f} s"
          + (f" ({pt / rt:.0f} times sooner)" if rt else ""))
    print(f"Rust over all {len(files)}: {sum(rust[p][1] or args.timeout for p in files):.2f} s; "
          "its slowest:")
    for p in sorted(files, key=lambda p: -(rust[p][1] or args.timeout))[:10]:
        t = python[p][1]
        print(f"  {rust[p][1] or math.inf:8.3f} s  (Python {'out of time' if t is None else f'{t:.1f} s'})  {p}")
    if late:
        print(f"not measured by Python in {args.timeout:g} s, and Rust's answer:")
        for p in late:
            ra, rt, rsaid = rust[p]
            if ra is None:
                said = rsaid
            elif "volume" in ra:
                said = f"volume {ra['volume']!r}, area {ra['area']!r}"
            else:
                said = json.dumps(ra)[:150]
            print(f"  {rt if rt is not None else math.inf:8.3f} s  {p}\n            {said}")
    sys.exit(1 if wrong else 0)


if __name__ == "__main__":
    main()
