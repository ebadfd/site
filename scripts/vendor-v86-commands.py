#!/usr/bin/env python3
"""Vendor a deterministic, local subset of copy.sh's Arch v86 filesystem.

Downloads selected commands, their ELF shared-library dependency closure, and
/etc files needed by libc/NSS. The full upstream filesystem is several GB and
is intentionally not mirrored; use vendor-v86-full.py for that. The output
directory is gitignored and excluded from the Docker build context.
"""

import json
import pathlib
import subprocess
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "static/v86/images/arch"
MANIFEST = ROOT / "static/v86/images/fs.json"
UPSTREAM = "https://i.copy.sh/arch/"
COMMANDS = {
    "cat", "clear", "cut", "date", "env", "find", "grep", "head",
    "hostname", "id", "ls", "printf", "ps", "pwd", "sed", "sort",
    "tail", "tr", "uname", "wc", "which", "whoami",
}


def download(url: str, target: pathlib.Path) -> None:
    if target.exists():
        return
    target.parent.mkdir(parents=True, exist_ok=True)
    with urllib.request.urlopen(url, timeout=120) as response:
        target.write_bytes(response.read())


def walk(nodes, parent=""):
    for node in nodes:
        path = f"{parent}/{node[0]}"
        yield path, node
        for value in node:
            if isinstance(value, list) and value and isinstance(value[0], list):
                yield from walk(value, path)


def main() -> None:
    download("https://i.copy.sh/fs.json", MANIFEST)
    manifest = json.loads(MANIFEST.read_text())
    nodes = dict(walk(manifest["fsroot"]))

    # Map library names to regular files, following the manifest's symlinks.
    by_name = {}
    for path, node in nodes.items():
        if path.startswith("/usr/lib/"):
            by_name.setdefault(pathlib.PurePosixPath(path).name, (path, node))

    def resolve(path: str):
        seen = set()
        while path not in seen:
            seen.add(path)
            node = nodes.get(path)
            if not node:
                return None
            value = node[6] if len(node) > 6 else None
            if isinstance(value, str) and value.endswith(".bin"):
                return path, node
            if isinstance(value, str):
                path = str((pathlib.PurePosixPath(path).parent / value))
                # Normalize simple relative symlink components.
                path = "/" + "/".join(part for part in path.split("/") if part not in ("", "."))
            else:
                return None
        return None

    queue = []
    for command in sorted(COMMANDS):
        resolved = resolve(f"/usr/bin/{command}")
        if resolved:
            queue.append(resolved)

    # libc consults these at runtime; NSS modules are loaded via dlopen rather
    # than represented in ELF NEEDED entries.
    for path, node in nodes.items():
        value = node[6] if len(node) > 6 else None
        if path.startswith("/etc/") and isinstance(value, str) and value.endswith(".bin"):
            queue.append((path, node))
        if path.startswith("/usr/lib/libnss"):
            resolved = resolve(path)
            if resolved:
                queue.append(resolved)

    visited = set()
    missing_dependencies = set()
    while queue:
        path, node = queue.pop()
        chunk = node[6]
        if chunk in visited or not chunk.endswith(".bin"):
            continue
        visited.add(chunk)
        target = OUTPUT / chunk
        download(UPSTREAM + chunk, target)

        result = subprocess.run(
            ["readelf", "-d", str(target)], capture_output=True, text=True
        )
        if result.returncode != 0:
            continue
        for line in result.stdout.splitlines():
            if "(NEEDED)" not in line:
                continue
            needed = line.split("[", 1)[1].split("]", 1)[0]
            candidate = by_name.get(needed)
            if not candidate:
                missing_dependencies.add(f"{path}: {needed}")
                continue
            resolved = resolve(candidate[0])
            if resolved:
                queue.append(resolved)
            else:
                missing_dependencies.add(f"{path}: {needed}")

    if missing_dependencies:
        details = "\n".join(f"  - {item}" for item in sorted(missing_dependencies))
        raise SystemExit(f"Unresolved v86 dependencies:\n{details}")

    print(f"Vendored {len(visited)} filesystem chunks in {OUTPUT}")


if __name__ == "__main__":
    main()
