# site

personal website and the blog site

## The in-browser terminal

The `tty` button boots a real 32-bit Arch Linux in the browser with
[v86](https://github.com/copy/v86), inside a sandboxed iframe (`static/v86/term.html`). Blog
posts are injected into the guest as `/root/blog/*.md`, so `cat`, `less` and `vim` work on them.

What is committed: `static/v86/build/` (`libv86.js`, `v86.wasm`) and `static/v86/bios/`.

What is **not** committed: the guest images under `static/v86/images/` — the Arch state snapshot
(~15 MB) and the 9p chunk mirror (~5.4 GB, ~70k files). Fetch them locally with:

```sh
python3 scripts/vendor-v86-full.py      # full mirror, resumable
python3 scripts/vendor-v86-commands.py  # or just a small subset of commands
```

### `V86_ASSET_BASE`

Where the emulator loads `arch_state-v3.bin.zst` and `arch/<chunk>.bin` from. Defaults to
`/static/v86/images/`, which is the local copy above. In production point it at the CDN:

```sh
V86_ASSET_BASE=https://cdn.example.com/v86/
```

Responses under `/static/v86/images/` are served `immutable` and skip compression, since the
chunk names are content hashes and the payloads are already compressed.
