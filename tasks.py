# coding: utf-8

import sys
import subprocess
from invoke import task
from pathlib import Path


REPO_HOME = Path.cwd()
PYTHON_TARGETS = [
    f"3.{x}-{arch}"
    for x in range(8, 11)
    for arch in ("32", "64")
]



def get_python_path(py_ident):
    try:
        return subprocess.check_output([
            "py",
            f"-{py_ident}",
            "-c",
            "import sys;print(sys.executable)"
        ]).decode(sys.getfilesystemencoding())
    except subprocess.CalledProcessError:
        pass


@task
def build_all(c, release=False):
    c.run(" ".join([
        "cargo",
        "build",
        "--all",
        "--release" if release else "",
    ]))
    print("Build all completed")


@task
def build_wheels(c, release=False, strip=False):
    i_args = {
        f'"{py_path}"': "i686-pc-windows-msvc" if ident.endswith("32") else "x86_64-pc-windows-msvc"
        for ident in PYTHON_TARGETS
        if (py_path := get_python_path(ident))
    }
    with c.cd(REPO_HOME / "docrpy"):
        for (pypath, arch) in i_args.items():
            build_command = " ".join([
                "maturin build",
                "--release" if release else "",
                "--strip" if strip else "",
                f"-i {pypath}"
            ])
            c.run(
                build_command,
                env={'CARGO_BUILD_TARGET': arch}
            )


@task
def copy_artifacts(c, release=False):
    print("Copying artifacts to dist folder...")
    REPO_HOME.joinpath("dist").mkdir(parents=True, exist_ok=True)
    subfolder = 'release' if release else 'debug'
    c.run(f"cp ./target/{subfolder}/*.exe ./dist")
    c.run(f"cp ./target/{subfolder}/*lib.dll ./dist")
    c.run("cp ./target/wheels/*.whl ./dist")


@task
def upload_wheels(c):
    with c.cd(REPO_HOME):
        c.run(r'twine upload  "./target/wheels/*" --non-interactive --skip-existing')
