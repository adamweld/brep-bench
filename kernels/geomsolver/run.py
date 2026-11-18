#!/usr/bin/env python3
"""The brep-bench runner for geomsolver, whose solids are written in Solvent.

    python3 kernels/geomsolver/run.py CASE OUT.stl [OUT.step]   # CASE is an id, e.g. boolean/napkin-ring
    python3 kernels/geomsolver/run.py --version

A geomsolver part is a Solvent document, so each case is one: `cases/<category>/<name_snake>.sv`
beside this file states the solid the way a Solvent user writes it, and names it `part`. The runner
compiles it as a user does, with geomsolver's own compiler, `solventc` (built by build.py): the
document is parsed, elaborated and solved, and `part` is built by the core's B-rep kernel and
written as STEP (`solventc DOC --solid part --step OUT.step`). Given OUT.step, that file is the
whole answer, and nothing is measured or meshed.

Where the compiler declines to write the solid as STEP (its words name a solid it reads only point
by point, with no exact construction), the runner says so in `step_error` and compiles the
document again for the core's own measures of `part` (`--json --where part`: its volume and area)
and its triangles (`--stl OUT.stl`). Where that is declined too, the case is refused in the
compiler's words.

A document that does not parse, elaborate or solve is a fault of the implementation, not a
refusal: the runner prints the compiler's report and exits non-zero, so the harness records a
crash. A file with no code refuses: its paragraph of comments beginning `Refused:` says why.
"""
import json
import os
import re
import signal
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SOLVENTC = os.path.join(HERE, "bin", "solventc")
# the solid each document names as its answer
PART = "part"
# a diagnostic's place, `path:line:col: `, which the reason leaves out
PLACE = re.compile(r"^\S+?:\d+:\d+: ")

if len(sys.argv) > 1 and sys.argv[1] == "--version":
    with open(os.path.join(HERE, "bin", "TAG")) as f:
        print(f"geomsolver {f.read().strip()} (solventc)")
    sys.exit(0)


class Fault(Exception):
    """The document itself is wrong: it did not parse, elaborate or solve."""


class Refused(Exception):
    pass


def pause():
    """Under the harness (`BREP_BENCH_WARM` set; SPEC.md, "A runner"), stops the runner until the
    harness continues it: once it has loaded and found its case, and once it has answered, so that
    the harness times the kernel alone, between the two."""
    if os.environ.get("BREP_BENCH_WARM") and hasattr(signal, "SIGSTOP"):
        sys.stdout.flush()
        os.kill(os.getpid(), signal.SIGSTOP)


def implementation(case_id):
    """The document implementing a case: cases/<category>/<name_snake>.sv."""
    category, name = case_id.split("/")
    path = os.path.join(HERE, "cases", category, name.replace("-", "_") + ".sv")
    if not os.path.exists(path):
        sys.exit(f"no implementation of {case_id}")
    return path


def refusal(source):
    """Why a file with no code refuses: its paragraph of comments that begins `Refused:`; None for
    a file with code."""
    lines = [line.strip() for line in source.splitlines()]
    if any(line and not line.startswith("//") for line in lines):
        return None
    words = [line[2:].strip() for line in lines]
    if "Refused:" not in " ".join(words):
        raise Fault("a file with no code must say why it refuses, in a paragraph beginning `Refused:`")
    start = next(i for i, w in enumerate(words) if w.startswith("Refused:"))
    paragraph = []
    for w in words[start:]:
        if not w:
            break
        paragraph.append(w)
    return " ".join(paragraph)[len("Refused:"):].strip()


def compile_(document, *outputs):
    """Run solventc over the document with these outputs. Returns its standard output where it
    succeeded; raises Refused, in the compiler's words, where the document solved but an output was
    declined, and Fault where the document itself failed."""
    run = subprocess.run([SOLVENTC, document, *outputs], capture_output=True, text=True)
    if run.returncode == 0:
        return run.stdout
    solved = re.search(r": solved\b", run.stdout) and run.returncode == 1
    if not solved:
        raise Fault(f"solventc exited {run.returncode}\n{run.stdout}{run.stderr}")
    errors = [PLACE.sub("", line) for line in run.stderr.splitlines() if "error[" in line]
    raise Refused("; ".join(errors) or run.stderr.strip() or "solventc declined the output")


def measured(document, out_stl):
    """The core's own measures of the part, and its triangles in out_stl."""
    report = json.loads(compile_(document, "--json", "--where", PART, "--solid", PART,
                                 "--stl", out_stl))
    positions = report["documents"][0]["positions"]
    return {"status": "ok", "volume": positions[f"{PART}.volume"],
            "area": positions[f"{PART}.area"]}


def build(document, out_stl, out_step):
    if out_step:
        try:
            compile_(document, "--solid", PART, "--step", out_step)
            return {"status": "ok"}
        except Refused as why:
            declined = str(why)
        reply = measured(document, out_stl)
        reply["step_error"] = declined
        return reply
    return measured(document, out_stl)


def main():
    if len(sys.argv) not in (3, 4):
        sys.exit("usage: run.py CASE OUT.stl [OUT.step]")
    case_id, out_stl = sys.argv[1], sys.argv[2]
    out_step = sys.argv[3] if len(sys.argv) == 4 else None
    document = implementation(case_id)
    with open(document) as f:
        why = refusal(f.read())
    pause()
    try:
        if why is not None:
            raise Refused(why)
        reply = build(document, out_stl, out_step)
    except Refused as why:
        reply = {"status": "refused", "reason": str(why)}
    except Fault as fault:
        sys.exit(str(fault))
    print(json.dumps(reply))
    pause()


if __name__ == "__main__":
    main()
