#!/usr/bin/env python3

# RCL -- A reasonable configuration language.
# Copyright 2024 Ruud van Asseldonk

# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# A copy of the License has been included in the root of the repository.

"""
Publish files tracked in the RCL (mono)repository into separate repositories.

Some tools -- in particular around Tree-sitter -- are repository-oriented and
expect some code that we rather track in subdirectories, to live in their own
repository. This script updates those external repositories to match the source
of truth in this repository.

USAGE

    tools/update_repos.py [tree-sitter-rcl] [zed-rcl]

REPOSITORIES

We expect the following directories to exist relative to the repository root:

  ../tree-sitter-rcl
  ../zed-rcl

RATIONALE

Why export to an external repository rather than including those external
repositories as a Git submodule here?

  * Git submodules currently do not work very nicely with Nix flakes, which
    would make it harder to use the current Nix-based CI.
  * Some consumers of Tree-sitter grammars prefer to have the generated files
    be part of the repository, but in this repository we prefer to be minimal
    and don't have large assets that are effectively opaque like binaries. In
    an external repository, we can have both.
"""

import shutil
import sys
import textwrap

from os import path
from subprocess import check_output
from typing import List


def ignores_tree_sitter_rcl(dirname: str, _entries: List[str]) -> List[str]:
    if dirname == "grammar/tree-sitter-rcl":
        return [".gitignore", "Cargo.rcl"]
    else:
        return []


def ignores_zed(dirname: str, _entries: List[str]) -> List[str]:
    if dirname == "grammar/zed":
        return [".gitignore", "extension.rcl", "grammars"]
    else:
        return []


def git_commit(repo_dir: str, message: str) -> None:
    print(f"Creating commit in {repo_dir} ...")
    git = ["git", "-C", repo_dir]
    check_output([*git, "add", "."])
    out = check_output([*git, "commit", "--message", message.strip()])
    print(out.decode("utf-8"))


def main() -> None:
    assert path.isdir("../tree-sitter-rcl/.git")
    assert path.isdir("../zed-rcl/.git")

    current_desc = check_output(["git", "describe", "--dirty"]).decode("utf-8")
    current_commit = check_output(["git", "rev-parse", "HEAD"]).decode("ascii")

    repos = sys.argv[1:]
    if repos == []:
        repos = ["tree-sitter-rcl", "zed-rcl"]

    if "zed-rcl" in repos:
        src_dir = "grammar/zed"
        dst_dir = "../zed-rcl"
        # TODO: It would be nice to skip the index and just generate the right
        # Git tree. This is possible with `git fast-import`, which is also what
        # MkDocs uses under the hood. But let's not go there right now.
        shutil.copytree(
            src=src_dir,
            dst=dst_dir,
            ignore=ignores_zed,
            dirs_exist_ok=True,
        )
        message = textwrap.dedent(
            f"""
            Update to RCL {current_desc}

            This is a generated export based on files in the {src_dir}
            directory of the main RCL repository. It is generated by
            tools/update_repos.py.

            Upstream-Commit: {current_commit}
            """
        )
        git_commit(dst_dir, message)

    if "tree-sitter-rcl" in repos:
        src_dir = "grammar/tree-sitter-rcl"
        dst_dir = "../tree-sitter-rcl"

        # Rebuild the Tree-sitter grammar to ensure that we don't copy anything
        # stale.
        check_output(["tree-sitter", "generate", "--build"], cwd=src_dir)

        shutil.copytree(
            src=src_dir,
            dst=dst_dir,
            ignore=ignores_tree_sitter_rcl,
            dirs_exist_ok=True,
        )
        message = textwrap.dedent(
            f"""
            Update to RCL {current_desc}

            This is a generated export based on files in the {src_dir}
            directory of the main RCL repository. It is generated by
            tools/update_repos.py.

            Upstream-Commit: {current_commit}
            """
        )
        git_commit(dst_dir, message)


if __name__ == "__main__":
    main()
