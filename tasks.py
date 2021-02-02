# coding: utf-8

from pathlib import Path
from invoke import task


REPO_HOME = Path.cwd()
PYTHON_TARGETS = [
    r"C:\python37-x64",
    r"C:\python37",
    r"C:\python38-x64",
    r"C:\python38",
    r"C:\python39-x64",
    r"C:\python39",
]


@task
def build_wheels(c):
    with c.cd(REPO_HOME / "docrpy"):
        pythons = [Path(pypath, "python.exe").resolve() for pypath in PYTHON_TARGETS]
        i_arg = " -i ".join(f'"{str(py)}"' for py in pythons)
        c.run(f"maturin build --release -i {i_arg}")
