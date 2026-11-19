#!/usr/bin/env python3
"""Build the brep-bench runner for geomsolver.

    python3 kernels/geomsolver/build.py

geomsolver's command-line compiler, `solventc`, is built from its source at the commit COMMIT
names (TAG says where it is: a release tag, or the branch it was pushed to), checked out into kernels/geomsolver/upstream/: it is what a Solvent
user runs to turn a `.sv` document into a solid and write it as STEP. It is the pure-Rust build,
the one the project releases: its own B-rep kernel (cad-kernel), with no Open CASCADE. Needs Rust
(`cargo`) and git. Standard library only.
"""
import os
import shutil
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
UPSTREAM = os.path.join(HERE, "upstream")
REPOSITORY = "https://github.com/bmander/geomsolver"
# v0.1.24 with faces deleted as sets built exactly and traced curves written as cubics; to be
# pinned to the release that carries them
TAG = "ccr-c93277d3-pzcw5v"
COMMIT = "a2596c13b741e7b80dcb55606e123648bf2afbdd"
# where the runner finds the compiler, and the tag it names in --version
BINARY = os.path.join(HERE, "bin", "solventc")


def git(*args):
    subprocess.run(["git", "-C", UPSTREAM, *args], check=True)


def main():
    have = subprocess.run(["git", "-C", UPSTREAM, "rev-parse", "HEAD"], capture_output=True,
                          text=True).stdout.strip() if os.path.isdir(os.path.join(UPSTREAM, ".git")) else ""
    if have != COMMIT:
        os.makedirs(UPSTREAM, exist_ok=True)
        if not have:
            git("init", "-q")
        git("fetch", "-q", "--depth", "1", REPOSITORY, COMMIT)
        git("checkout", "-q", "--detach", "FETCH_HEAD")
    manifest = os.path.join(UPSTREAM, "rust", "Cargo.toml")
    build = subprocess.run(["cargo", "build", "--release", "--manifest-path", manifest,
                            "-p", "gcs-cli"])
    if build.returncode:
        sys.exit(build.returncode)
    os.makedirs(os.path.dirname(BINARY), exist_ok=True)
    shutil.copy2(os.path.join(UPSTREAM, "rust", "target", "release", "solventc"), BINARY)
    with open(os.path.join(HERE, "bin", "TAG"), "w") as f:
        f.write(f"{TAG} {COMMIT[:12]}\n")


if __name__ == "__main__":
    main()
