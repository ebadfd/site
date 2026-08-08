#!/usr/bin/env python3
"""Resumably mirror the complete copy.sh Arch v86 9p filesystem.

The result can be uploaded as-is to a CDN and pointed at with $V86_ASSET_BASE; the
output directory is gitignored and excluded from the Docker build context. Files are
stored by their manifest chunk names under static/v86/images/arch/. Existing files with
the expected size are skipped; partial downloads use .part files and atomic renames.
"""

import concurrent.futures
import json
import os
import pathlib
import time
import urllib.error
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "static/v86/images/fs.json"
OUTPUT = ROOT / "static/v86/images/arch"
UPSTREAM = "https://i.copy.sh/arch/"
WORKERS = int(os.environ.get("V86_DOWNLOAD_WORKERS", "16"))
RETRIES = 5


def walk(nodes):
    for node in nodes:
        yield node
        for value in node:
            if isinstance(value, list) and value and isinstance(value[0], list):
                yield from walk(value)


def fetch(item):
    name, expected_size = item
    target = OUTPUT / name
    if target.exists() and target.stat().st_size == expected_size:
        return "skipped", expected_size

    partial = target.with_suffix(target.suffix + ".part")
    for attempt in range(RETRIES):
        try:
            request = urllib.request.Request(
                UPSTREAM + name,
                headers={"User-Agent": "ebadfd-v86-mirror/1.0"},
            )
            with urllib.request.urlopen(request, timeout=120) as response:
                data = response.read()
            if len(data) != expected_size:
                raise IOError(
                    f"size mismatch for {name}: expected {expected_size}, got {len(data)}"
                )
            partial.write_bytes(data)
            partial.replace(target)
            return "downloaded", expected_size
        except (OSError, urllib.error.URLError) as error:
            partial.unlink(missing_ok=True)
            if attempt + 1 == RETRIES:
                return "failed", f"{name}: {error}"
            time.sleep(2 ** attempt)


def main():
    if not MANIFEST.exists():
        raise SystemExit(f"Missing manifest: {MANIFEST}")

    manifest = json.loads(MANIFEST.read_text())
    chunks = {}
    for node in walk(manifest["fsroot"]):
        value = node[6] if len(node) > 6 else None
        if isinstance(value, str) and value.endswith(".bin"):
            size = int(node[1])
            previous = chunks.get(value)
            if previous is not None and previous != size:
                raise SystemExit(f"Conflicting sizes for {value}: {previous} and {size}")
            chunks[value] = size

    OUTPUT.mkdir(parents=True, exist_ok=True)
    total = len(chunks)
    complete = downloaded = failed = bytes_done = 0
    failures = []

    with concurrent.futures.ThreadPoolExecutor(max_workers=WORKERS) as executor:
        for status, value in executor.map(fetch, sorted(chunks.items())):
            complete += 1
            if status == "downloaded":
                downloaded += 1
                bytes_done += value
            elif status == "failed":
                failed += 1
                failures.append(value)

            if complete % 500 == 0 or complete == total:
                print(
                    f"{complete}/{total} checked; {downloaded} downloaded; "
                    f"{failed} failed; {bytes_done / 1_000_000:.1f} MB fetched",
                    flush=True,
                )

    if failures:
        report = ROOT / "static/v86/images/failed-chunks.txt"
        report.write_text("\n".join(failures) + "\n")
        raise SystemExit(f"{len(failures)} downloads failed; see {report}")

    (ROOT / "static/v86/images/failed-chunks.txt").unlink(missing_ok=True)
    print(f"Complete: {total} chunks are available in {OUTPUT}")


if __name__ == "__main__":
    main()
