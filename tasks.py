# coding: utf-8

import os
import struct
from pathlib import Path
from invoke import task


REPO_HOME = Path.cwd()
ARCH = "x86" if struct.calcsize("P") == 4 else "x64"
PYTHON_TARGETS = {
    "x86": [
        r"C:\python37",
        r"C:\python38",
        r"C:\python39",
    ],
    "x64": [
        r"C:\python37-x64",
        r"C:\python38-x64",
        r"C:\python39-x64",
    ]
}


@task
def build_wheels(c):
    with c.cd(REPO_HOME / "docrpy"):
        pythons = [Path(pypath, "python.exe").resolve() for pypath in PYTHON_TARGETS[ARCH]]
        i_arg = " -i ".join(f'"{str(py)}"' for py in pythons)
        c.run(f"maturin build --release -i {i_arg}")


@task
def upload_wheels(c):
    tag_triggered = os.environ.get('APPVEYOR_REPO_TAG_NAME', "").startswith("release")
    if not tag_triggered:
        return print("Not a release build.\nSkipping PyPI upload process.")
    with c.cd(REPO_HOME):
        c.run('twine upload  ".\target\wheels\*" --non-interactive --skip-existing')
