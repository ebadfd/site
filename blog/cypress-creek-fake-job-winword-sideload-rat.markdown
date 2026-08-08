---
title: "The Fake Job Offer That Hid a RAT"
date: "2026-06-17"
image: "https://cdn.ebadfd.tech/cypress-creek-fake-job/cover.jpg"
about: "A fake LinkedIn recruiter, a 'salary & benefits' PDF that was really an EXE, and a Quasar RAT hiding inside Windows Calculator. We follow a DPRK-style fake-job lure from the chat message all the way down to a live C2 — and show where the encryption finally stopped us."
tags:
  - "malware-analysis"
  - "reverse-engineering"
  - "infosec"
  - "dprk"
  - "rat"
---

This one didn't start with a file on a sandbox — it started with a LinkedIn chat.

A friend, **Pulasthi Ranabahu**, got messaged by a "recruiter" hiring for **Cypress Creek Renewables**, a real US renewable-energy company. The recruiter's profile looked the part: polished photo, a little verified-style badge, a solid-looking career history. He passed Pulasthi a job application form, a glossy "salary & benefits" pack, and eventually some files to "just check." Pulasthi noticed the files were `.exe`s, laughed it off ("*I can't run exe in android 😂 — why is it an exe file?*"), and sent the whole thing to us instead. Good instinct — that "benefits structure" was a remote-access trojan.

> There's a major scam going around right now, and undergraduates are the main targets. The accounts look credible — impressive degrees, lots of work experience, the works. Always verify an opportunity before you share documents or personal details with someone you met online. Think before you trust. — the heads-up from Pulasthi that kicked this off.

This is a textbook **DPRK "Contagious Interview" / Famous Chollima** play: pose as a recruiter, wrap malware in a benefits document, and let curiosity do the clicking. This post follows the whole thing — from that LinkedIn message, through a legitimately-signed Microsoft binary, a 100 MB DLL, an embedded Python runtime, a .NET RAT living inside Windows Calculator, all the way to a live C2 — and an honest account of where the encryption finally stopped us.

This research was conducted by the team at [Loomzy](https://loomzy.io/security).

---

## How It Arrived

The approach came from a LinkedIn account using the name **"Adham Rafeek"** — recruiter persona, verified-looking badge, the usual confidence. The pitch was friendly and specific: here's the application form, here's the company, read the PDF carefully before you apply.

![The fake recruiter sharing the application form and 'benefits' PDF on LinkedIn](https://cdn.ebadfd.tech/cypress-creek-fake-job/linkedin-chat-01.jpg)

The form link (`https://forms.gle/myFYMh2APifLekcr9`) led to a clean, **Cypress Creek-branded Google Form** — "Build a Better Tomorrow," innovative solar projects, strong team culture — linking out to a "Job Description & Project Overview" document and asking for your details:

![Cypress Creek branded Google Form used as the lure front-end](https://cdn.ebadfd.tech/cypress-creek-fake-job/google-form.jpg)

When Pulasthi said he didn't have the experience, the "recruiter" did the classic pressure flip — *"Are our salary and benefits not reasonable for you? Someone will guide you through the process"* — and started sending files. LinkedIn itself flagged them ("*This message may contain unwanted or harmful content*"), and shortly after, the account was gone: name stripped to "LinkedIn Member," **"unable to receive messages"** — taken down.

![LinkedIn flagging the files as harmful; the account later banned](https://cdn.ebadfd.tech/cypress-creek-fake-job/linkedin-chat-02.jpg)

The actual payload was delivered as a download disguised as a PDF:

```text
https://www.dropbox.com/.../Cypress-Creek-Company.Pdf.rar   ← note the double extension
```

A `.Pdf.rar` — a RAR archive wearing a PDF name. Inside was the "recruitment pack." That archive is where the technical story begins.

---

## The Lure Package

The folder layout is pure social engineering — three numbered directories that read like an onboarding pack:

```
01_Company and project introduction/   *.docx   (benign decoys)
02_Job_Description/                     *.docx   (benign decoys — SEO, PPC, Google Ads…)
03_Benefits, Salary and Responsibility/
    1.Cypress Creek Salary, Bonus, and Benefits Structure.exe
    msvcr100.dll
    word.dll
    Winhttp2
```

`file` on the payload directory is where it gets interesting:

```text
1.Cypress…Structure.exe : PE32+ executable (GUI) x86-64
word.dll                : PE32+ executable (DLL) x86-64, 9 sections
msvcr100.dll            : PE32+ executable (DLL) x86-64  — 104,857,600 bytes (exactly 100 MB)
Winhttp2                : data  — 21,149,421 bytes, no header, entropy 8.0
```

A 100 MB `msvcr100.dll` (the real Microsoft one is ~800 KB) and a 21 MB headerless high-entropy blob named like a Windows component. Neither of those is normal. The `.docx` files are exactly what they claim to be — decoys to keep the victim reading while the binary runs.

---

## Stage 1 — A Genuine, Signed `WinWord.exe`

The EXE is the file the victim double-clicks, but it is **not** the malware. Its version resource gives it away:

```text
OriginalFilename : WinWord.exe
ProductName      : Microsoft Office 2013
FileVersion      : 15.0.4569.1504
CompanyName      : Microsoft Corporation
```

It's an unmodified, **Microsoft-signed** Word launcher stub (Authenticode chains to *Microsoft Code Signing PCA*), renamed to the salary lure. Its `.text` is a tiny ~3.8 KB of standard MSVC CRT startup. Disassembling the entry point shows nothing but `__security_init_cookie` → `mainCRTStartup` → IAT calls. There is zero malicious code here.

What makes it the perfect host is its import table:

```text
DLL Name: KERNEL32.dll
DLL Name: MSVCR100.dll   ← loaded from the local directory first
```

A thin signed launcher that imports `MSVCR100.dll` and immediately calls into it during CRT init. Drop a malicious `msvcr100.dll` next to it and **DLL search-order hijacking** does the rest — the payload detonates during DLL load, before `main` even matters. The signed binary never has to do anything wrong.

---

## The Proxy Trick — Which DLL Is Actually Malicious?

Our first instinct was that `word.dll` was the loader. It wasn't — and the way we figured that out is worth showing.

`msvcr100.dll`'s exports are **all forwarders**:

```text
msvcr100.dll → word.$I10_OUTPUT
msvcr100.dll → word.??0exception@std@@QEAA@XZ
msvcr100.dll → word.??0bad_cast@std@@…
… (every CRT symbol forwards to word.<name>)
```

So `msvcr100.dll` is a **proxy**: it satisfies WinWord's CRT imports by forwarding them to `word.dll`, keeping the app alive. We loaded `word.dll` into radare2 and chased every "suspicious" API:

```text
CreateProcessW → sym.MSVCR100.dll__wpopen        (legit _popen)
CreatePipe     → sym.MSVCR100.dll__pipe           (legit)
VirtualAlloc   → sym.MSVCR100.dll__resetstkoflw   (legit stack-overflow reset)
```

Every one resolved to a **genuine CRT function**. `word.dll` is the *real* MSVC 2010 runtime, renamed. The malware is hiding in the component we'd dismissed — the 100 MB `msvcr100.dll` proxy. The decisive tell:

```text
msvcr100.dll imports: KERNEL32.dll, ntdll.dll, Shlwapi.dll
word.dll     imports: KERNEL32.dll
```

The genuine VS2010 `msvcr100.dll` never imports `Shlwapi` (path-building APIs). And its real imports — `GetLocaleInfoEx`, `InitializeCriticalSectionEx`, `FlsAlloc`, `InitializeSListHead` — are **VS2015+ APIs** that the 2010 runtime never used. This "msvcr100.dll" is a freshly-compiled modern C++ loader wearing the runtime's name. The other 99.5 MB? Pure overlay padding appended after the PE — a crude but effective trick to push the file past the size limit of many AV engines and sandbox uploaders.

```text
msvcr100.dll real PE: ~445 KB     overlay padding: ~99.5 MB
```

---

## Inside the Loader

`msvcr100.dll`'s `DllMain` is a standard `_DllMainCRTStartup`. The malicious code runs as a **CRT static initializer** — radare2 lays out the flow cleanly:

```text
fcn.18001ab90  (static init, reason == 1)
   → fcn.180015aa0(&ctx)   read payload file + derive key
   → fcn.180016770(&ctx)   decrypt → extract → deploy → inject
```

The loader's import surface is exactly a loader's toolkit: `CreateFileW`, `GetFileSizeEx`, `SetFilePointerEx`, `ReadFile`, `CreateDirectoryW`, `CopyFileW`, `VirtualProtect`, and `LoadLibraryW`+`GetProcAddress` for resolving `Shlwapi` path functions at runtime (which is why `C:\ProgramData\…` paths never appear as static strings).

The transform routine, `fcn.180015aa0`, is deliberately ugly — floating-point opaque predicates and byte-table-driven branches designed to waste a reverser's afternoon:

```asm
cvtdq2pd xmm0, xmm0          ; junk FP math as opaque predicates
mulsd    xmm1, xmm6          ; constants 128.0, 136.0 …
movsx    edx, byte [0x18006a048]   ; control-flow from [1,3,7,8,9] table
div      ecx
```

We checked for known ciphers and found **nothing** — no AES S-box, no ChaCha `expand 32-byte k`, no SHA/MD5 constants. `.data` instead held what looked like Huffman/LZ length tables, hinting at a custom decrypt-plus-decompress. In other words: a bespoke, obfuscated transform with no off-the-shelf shortcut.

### The `Winhttp2` blob

```python
size: 21,149,421 bytes  (% 16 = 13 → not block-aligned)
entropy (whole)      : 8.000   # fully random
entropy (first 256B) : 7.226   # slight header
repeated 16-byte blocks: none  # rules out AES-ECB
```

Not block-aligned, no ECB repetition, 8.0 entropy throughout — a **stream cipher** (or custom transform) over a payload that, once decrypted, is large enough to be a runtime. We tried the obvious keyings (header-as-key RC4/XOR, embedded-PE scan) — all dead ends, as expected for genuinely-encrypted data.

### Letting the malware do the work

Rather than hand-deobfuscate the loader, we started building a **Unicorn** emulation harness — map the PE, stub the Win32 APIs, feed `Winhttp2` through the file-read hooks, and capture whatever the loader writes. That's the right move when the cipher is custom and the loader is obfuscated: don't reverse the math, let the code decrypt for you. But before finishing the harness, dynamic analysis gave us the answer faster.

---

## Detonation — The Real Chain

Running the sample (`infected`-grade caution applies) in **ANY.RUN** unrolled the chain in seconds. The process tree:

```text
WinRAR.exe  data.zip
└─ 1.Cypress Creek Salary, Bonus, and Benefits Structure.exe   (renamed WinWord.exe)
   └─ pythonhw.exe  "C:\ProgramData\Zhujikdo\Lib\Image.png"  "Zhujikdo"
      └─ calc.exe    (injected — beacons to C2)
   └─ (decoy) Acrobat.exe  Job_Infomation.pdf
```

And the file events explained the 21 MB:

```text
extract: C:\ProgramData\Zhujikdo\python310.zip  →  C:\ProgramData\Zhujikdo\
         + ctypes/macholib, site-packages/…
launch : C:\ProgramData\Zhujikdo\pythonhw.exe  Lib\Image.png  Zhujikdo
```

`Winhttp2` decrypts to a **bundled CPython 3.10 embeddable runtime** plus an **InvisibleFerret** Python script disguised as `Image.png`. `pythonhw.exe` is a renamed `python.exe`. That Python loader/stealer then **injects a second-stage RAT into `calc.exe`**, which is what opens the network connection. The Python stage explains the size; the injected `calc.exe` explains the beacon.

---

## The Implant — A Quasar-Family .NET RAT

The free ANY.RUN tier won't let you download dropped files, but it *will* hand you **process memory dumps**. We pulled the `calc.exe` regions (password `infected` on the AES-zipped exports — needs `7z`, not `unzip`) and went hunting.

The C2 endpoint was sitting in memory as a raw `sockaddr_in` and as a string:

```text
0xc8540:  02 00 | da c1 | ce ce 4d 8a    → AF_INET, port 56001, 206.206.77.138
utf-16  : "206.206.77.138:56001"
```

And the implant fingerprint is unmistakable:

```text
protobuf-net, Version=2.4.0.0          ← Quasar packet serialization
Gma.System.MouseKeyHook                 ← Quasar keylogger
AesCryptoServiceProvider / GenerateIV   ← AES message crypto
SecureCredential / client certificates  ← SslStream TLS channel
Kly2q36xmILwD8YNfBO, n6S13eKWW0hHcQAn0pq ← ConfuserEx-style obfuscation
```

`protobuf-net` + `Gma.System.MouseKeyHook` + AES + a long-lived self-signed TLS cert is the signature of **QuasarRAT** (or a close fork). The runtime dump also leaked the operator-facing identity and the loot targets in cleartext:

```text
Client ID : admin[DESKTOP-JGLLJLD]          (Quasar username[hostname] format)
Mutex     : Zgtwknnvr
Keylog    : %Temp%\21232f297a57a5a743894a0e4a801fc3.dat   (= MD5("admin"))
Wallets   : Binance Chain, Coinbase, Coin98, Guarda, Braavos, Math, Liquality, iWallet…
Browsers  : Brave, CocCoc, 360, Amigo, Citrio, Comodo Dragon, Chedot, Mail.Ru Atom… (30+)
AV recon  : \\DESKTOP-JGLLJLD\ROOT\SecurityCenter2:AntiVirusProduct
```

So even without touching the network, the dump told us *what* the C2 does: keylogging, browser-credential and crypto-wallet theft, remote shell, and full remote control.

---

## The Traffic — TLS All the Way Down

The phishing tester also handed us the connection captures (`data*.bin`) and a full PCAP. Parsing the TLS records of the C2 stream:

```text
data(1).bin → Handshake(90)                         ClientHello
data.bin    → Handshake(1978)                       ServerHello + Certificate
data(2).bin → Handshake(102), CCS(1), Handshake(48) ClientKeyExchange, Finished
data(3).bin → CCS(1), Handshake(48)
```

The C2 channel is **raw TLS on port 56001** — not HTTP. The server certificate is self-signed and absurd:

```text
Serial    : 0x009eef6cd8775b40f9d4b969d4401399   (matches the calc.exe dump)
Subject=Issuer : CN=Fgvgymyktx
Algorithm : sha512WithRSAEncryption, RSA-2048
Valid     : 2026-05-04  →  2088-05-06   (62-year validity — the RAT tell)
```

```text
JA3  (client) : fc54e0d16d9764783542f0146a98b300
JA3S (server) : b74704234e6128f33bff9865696e31b3
Cipher        : 0xc014 = ECDHE-RSA-AES256-CBC-SHA (TLS 1.0)
```

The full PCAP showed a **220-second hands-on session**: 151 KB victim→C2 (exfil) and 251 KB C2→victim (commands and tooling). Not a fire-and-forget beacon — an operator was at the keyboard.

---

## Chasing the Master Secret

Here is the honest part. We *have* the encrypted command traffic, but it is **ECDHE** — perfect forward secrecy means the server's key can't recover it; only the session secrets can. ANY.RUN's SSL-key export is a paid feature, and the capture is legacy `.pcap` with no embedded `CLIENT_RANDOM` block. So the wire is sealed.

Except — we noticed both the client and server randoms were sitting in the `calc.exe` dump:

```python
client_random 6a31927e472980a1… → in calc dump: YES @0x30581b
server_random 6a31927e5bcb0aeb… → in calc dump: YES @0x305b0f
```

If the randoms are in-process, maybe the 48-byte TLS master secret is too. We wrote a verifier that, for every candidate window in the dump, derives the TLS 1.0 key block and checks whether it correctly HMACs the first captured record:

```python
def test(ms):
    kb = prf10(ms, b"key expansion", server_random + client_random, 136)
    cmac, ckey, civ = kb[0:20], kb[40:72], kb[104:120]
    pt = AES.new(ckey, AES.MODE_CBC, civ).decrypt(rec0)
    # validate PKCS7 padding, then verify HMAC-SHA1 over seq||hdr||content
    …
```

We scanned all four dumps — ~1.2 million high-entropy candidates:

```text
dump-…f47 (32 MB calc.exe) : 731,515 candidates → not found
dump-…ee5 (3.3 MB)         : 180,774          → not found
dump-…dab / …db9 (564 KB)  : 141,508 each     → not found
→ MASTER SECRET NOT PRESENT in any dump
```

The verdict: the implant uses **.NET `SslStream` → Windows SChannel**, so the master secret lives in **`lsass.exe`**, never in the malware's own process. The randoms are mirrored into the app; the secret is not. To finish the decryption we'd need an `lsass` memory dump (obtainable the same way as the calc.exe dumps) or a local re-run with `SSLKEYLOGFILE` set — at which point our verifier writes a key log and `tshark -o tls.keylog_file:…` peels the TLS, exposing the Quasar protobuf underneath.

We didn't need it to understand the C2 — the process dump already gave us the plaintext substance — but it's the one clean layer left to crack, and it's a process-dump away.

---

## What's Still Locked

In the spirit of full disclosure:

- **The on-disk `Winhttp2` cipher** — we identified it as a bespoke, obfuscated stream transform with no standard constants, and chose to recover the payload dynamically (the malware decrypts to `C:\ProgramData\Zhujikdo\`) rather than hand-reverse the FP-junk math. A finished Unicorn harness would extract it statically.
- **The live C2 command bytes** — sealed behind ECDHE TLS whose keys are in `lsass`. The capabilities and config are recovered; the exact command transcript needs an `lsass` dump or `SSLKEYLOGFILE`.

---

## The Full Chain

```text
LinkedIn "Adham Rafeek"  →  Google Form  →  Cypress-Creek-Company.Pdf.rar (Dropbox)
└─ 1.Cypress…Benefits Structure.exe   STAGE 1: signed WinWord.exe 2013, renamed
   └─ msvcr100.dll                     PROXY: forwards CRT to word.dll, 100 MB padded
      └─ word.dll                       genuine MSVCR100 runtime (decoy)
      └─ (malicious loader code)        modern VC++ in the padded msvcr100.dll
         └─ Winhttp2                     STAGE 2: encrypted CPython 3.10 + InvisibleFerret
            └─ pythonhw.exe Image.png    InvisibleFerret loader/stealer (Python)
               └─ inject → calc.exe      STAGE 3: Quasar-family .NET RAT
                  └─ TLS → 206.206.77.138:56001   C2 (keylog, creds, wallets, shell)
```

---

## IOCs

### Delivery

```text
LinkedIn persona   "Adham Rafeek" (recruiter, verified-style badge — account banned)
Lure form          https://forms.gle/myFYMh2APifLekcr9  (Cypress Creek Google Form)
Payload archive    Cypress-Creek-Company.Pdf.rar  (Dropbox, double-extension)
```

### Network

```text
C2                 206.206.77.138:56001  (TCP/TLS, ASN REGXA-CLOUD)
TLS cert serial    0x009eef6cd8775b40f9d4b969d4401399
TLS cert CN        Fgvgymyktx  (self-signed, valid until 2088-05-06)
JA3  (client)      fc54e0d16d9764783542f0146a98b300
JA3S (server)      b74704234e6128f33bff9865696e31b3
```

### Files (SHA-256)

```text
1.Cypress…Structure.exe  1260eed47c7e8200b78673397274e859bd1a3a7abe6ec8c6880dbe5fdccfe068
msvcr100.dll (loader)    c92773274defa4b9ec83b8f6809ee1fb41d294c3469229339c62f3260c978d15
word.dll (genuine CRT)   ae3cb6c6afba9a4aa5c85f66023c35338ca579b30326dd02918f9d55259503d5
Winhttp2 (stage 2)       70e8df089b23c16afcf9e1d489218cc630cf2097131999eb33239d096fef7034
```

### Host

```text
C:\ProgramData\Zhujikdo\                       deployment directory
C:\ProgramData\Zhujikdo\pythonhw.exe           renamed Python 3.10 interpreter
C:\ProgramData\Zhujikdo\Lib\Image.png          InvisibleFerret payload (Python)
C:\ProgramData\Zhujikdo\python310.zip          embedded CPython stdlib
%Temp%\21232f297a57a5a743894a0e4a801fc3.dat    keylog (filename = MD5 of username)
Mutex: Zgtwknnvr
Client ID: admin[DESKTOP-JGLLJLD]
```

---

## Tools We Used

| Tool | Purpose |
|------|---------|
| `objdump` / `radare2` | PE triage, import/export mapping, locating the loader vs the genuine runtime |
| `nix-shell -p p7zip` | Extracting the AES-encrypted (`infected`) process dumps |
| `tshark` | Parsing TLS records, extracting cert / JA3 / randoms from the PCAP |
| `findms.py` (custom) | TLS 1.0 key-block derivation + HMAC verifier to brute the master secret from memory |
| Unicorn + Capstone | (Started) emulation harness to decrypt `Winhttp2` without executing the loader |
| ANY.RUN | Detonation, process dumps, connection captures |

---

## Key Takeaways

1. **The first exploit is the profile, not the binary.** A confident "recruiter" with a verified-looking badge and a great CV is the actual zero-day here. Verify the company and the person before you download anything — and a job's "benefits PDF" should never be an `.exe`.
2. **The signed binary is bait, not the threat.** Stage 1 here is a clean, Microsoft-signed `WinWord.exe`. Hunt the DLLs it side-loads, not the EXE.
3. **File size is a signal.** A 100 MB `msvcr100.dll` is padding-for-evasion; a 21 MB headerless 8.0-entropy blob is a packed runtime. Both scream before you disassemble anything.
4. **Forwarder DLLs hide the real loader.** When every export forwards elsewhere, the proxy's *own* `DllMain`/static-init is where the code lives — and its import list (here, `Shlwapi` + VS2015 APIs) betrays that it isn't the runtime it claims to be.
5. **Let the malware decrypt for you.** A bespoke obfuscated cipher isn't worth hand-reversing when a sandbox writes the plaintext (`Image.png`, `python310.zip`) to disk.
6. **Process dumps beat the wire.** TLS + AES protect data *in transit*; the implant decrypts everything into its own memory to use it. The C2, cert serial, wallet targets, and client ID were all plaintext in the `calc.exe` dump.
7. **ECDHE means the keys aren't in the PCAP.** For SChannel-based .NET implants, the TLS master secret is in `lsass` — dump that process, not just the malware's.

---

## Sample

The sample set (lure EXE, `msvcr100.dll` loader, genuine `word.dll`, and the encrypted `Winhttp2` stage 2) is available to fellow researchers:

[infected.zip](https://cdn.ebadfd.tech/cypress-creek-fake-job/infected.zip) (password: `infected`)

---

<alert type="success" message="Looking for a cybersecurity partner? We help organizations run audits, penetration tests, and security engagements." > </alert>

Reach out to us at [hello@loomzy.io](mailto:hello@loomzy.io) to get started.

---

*Analysis conducted June 2026. The C2 at `206.206.77.138:56001` was reachable at the time of writing. This is a DPRK-style "Contagious Interview" lure (InvisibleFerret + a Quasar-family RAT); if a recruiter sends you a "benefits structure" you have to run, don't.*

*Cover artwork generated by Gemini Nano Banana.*
