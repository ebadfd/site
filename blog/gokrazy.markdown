---
title: gokrazy is really cool
date: 2023-09-20
tags:
  - go
  - gokrazy
  - linux
skip_ads: true
---

When you deal with Linux, you end up hearing about "distributions" as different "flavors" of Linux combined with a bunch of other tools. This is mostly true, but it's slightly missing the forest for the trees.

Consider this famous and often misunderstood quote by Richard Stallman:

> I'd just like to interject for a moment. What you're referring to as Linux is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux.Linux is not an operating system unto itself, but rather another free component of a fully functioning GNU system made useful by the GNU corelibs, shell utilities and vital system components comprising a full OS as defined by POSIX.

<xeblog-hero ai="Nikon D3300, 35mm f/1.8 DX, a red formula one style car speeds along a racetrack with visible tire skidmarks. Photo by Xe Iaso" file="vroom"></xeblog-hero>

Many pages of ink have been spilled over analyzing this quote, and a lot of them fall short of really getting at the heart of the matter. What this actually means is something like this:

By itself, Linux is useless. It does boot the system, it does interface with hardware, but without a bunch of other tools, it's not very useful. It's like a car without a steering wheel, or a boat without a rudder. It does something, but it's not very useful. The real value of things like the GNU project, systemd, openrc and other tools in that vein is that they make Linux useful. They make it into a complete system that you can use to do things. They are the proverbial steering wheel and rudder in the metaphor.

<xeblog-conv name="Mara" mood="hacker" standalone>Fun fact, if you try to boot a Linux kernel without an init process, it'll just panic and crash!</xeblog-conv>

Most Linux systems on the face of the planet are built with GNU tools and utilities. In order to compile the Linux kernel, you need to use [GCC](https://gcc.gnu.org/). In order to run `ls` to list files in the current directory, you need to use [GNU coreutils](https://www.gnu.org/software/coreutils/coreutils.html). Every dynamically linked program uses [glibc](https://www.gnu.org/software/libc/) for performing basic system interactions like writing to files or opening network sockets. Everything is built on top of the GNU toolset. This is why Stallman is so adamant about calling it GNU/Linux. It's not that he's trying to take credit for Linux, it's that he's trying to give credit to the GNU project for making Linux useful.

However, there's a lot of room for nuance here. For example, [Alpine Linux](https://alpinelinux.org/) is a Linux distribution that uses [musl libc](https://musl.libc.org/) instead of [glibc](https://www.gnu.org/software/libc/) and [busybox](https://en.wikipedia.org/wiki/BusyBox) instead of GNU coreutils. It's still a Linux distribution, but it doesn't use the GNU toolset. It's still a Linux distribution, but it's not GNU/Linux.

<xeblog-conv name="Mara" mood="hacker" standalone>Also, for the record you can build the Linux kernel with clang, but that's a whole other can of worms. For one, GCC supports many more targets than clang likely ever will, but in general there are some compromises you need to make until clang implements some GCC-specific compiler extensions a bit better. Google, Facebook, and a few other companies do run LLVM compiled kernels in production though, so it's probably closer to viable than you think. Especially if you use ChromeOS or Android.</xeblog-conv>

So, what is a Linux distribution? It's a collection of tools that make Linux useful. It's a collection of tools that make Linux into a complete system. It's not a "flavor" of Linux (though this conceptually can exist with alternative kernels like the Zen kernel patchset), it's a system that just so happens to make Linux useful.

As a counter-argument, consider the reason why Linux runs on more devices worldwide than there are people: [Android](<https://en.wikipedia.org/wiki/Android_(operating_system)>). Android does use the Linux kernel, but it doesn't use any GNU tools in the stack at all. You can't take programs that are compiled against other Linux distributions and run them on Android. You can't take programs that are compiled against Android and run them on other Linux distributions.

<xeblog-conv name="Aoi" mood="wut">Wait, so does this mean Android's not a Linux distribution? What is it then?</xeblog-conv>

I'm going to argue that Android is not a Linux distribution unto itself. Android is a Linux _implementation_. It uses the Linux kernel, but that's where the similarities with the rest of the ecosystem end. Android is its own little world where there's just enough system tools to get the system running, but once you get into the UI, it's a completely different world. It's a completely different ecosystem. It's a completely different operating system.

<xeblog-conv name="Aoi" mood="wut">So what's the difference between a Linux distribution and a Linux implementation?</xeblog-conv>
<xeblog-conv name="Cadey" mood="enby">It's a bit of a fuzzy line, but I'd say that a Linux distribution is a collection of discrete tools that make Linux useful, and a Linux implementation is a cohesive collection of bespoke tools that make Linux into a complete system. Really, you could argue that if it has `/bin/sh`, it's a Linux distribution.</xeblog-conv>

## gokrazy

[gokrazy](https://gokrazy.org) is a Linux implementation that I've used off and on for a few years now. It's a very interesting project because everything on the system is written in Go save the kernel. The init process is in Go (and even listens over HTTP to handle updates!), every userland process is written in Go, and even the core system services are written in Go.

Out of the box a gokrazy install comes with these basic tools:

- The `init` process that is mandated to be the parent of all userland processes by the Linux kernel.
- A [DHCP](https://en.wikipedia.org/wiki/Dynamic_Host_Configuration_Protocol) client that automatically configures the network interface.
- A [NTP](https://en.wikipedia.org/wiki/Network_Time_Protocol) client that automatically sets the system clock.
- A little tool to save randomness from the kernel to a file so that it can be used to seed the random number generator on boot (because the Raspberry Pi doesn't have a robust hardware random number generator)

That's it. Everything else from the web UI to A/B update logic is written in Go. It boots in literal seconds, uses an insanely small amount of RAM out of the box, and runs with nearly zero overhead. When you configure your gokrazy install to run additional software, you do so by adding the Go command path to a configuration file and then updating to trigger a reboot into the new version.

Here's an example of what my gokrazy virtual machine's file tree looks like:

```
/ # tree etc gokrazy user
etc
├── breakglass.authorized_keys
├── gokr-pw.txt
├── gokrazy
│   └── sbom.json
├── hostname
├── hosts
├── http-port.txt
├── https-port.txt
├── localtime
├── machine-id
├── resolv.conf -> /tmp/resolv.conf
└── ssl
    └── ca-bundle.pem

gokrazy
├── dhcp
├── heartbeat
├── init
├── ntp
└── randomd

user
├── breakglass
├── fbstatus
├── qemu-guest-kragent
├── serial-busybox
├── tailscale
├── tailscaled
└── waifud-gok-agent
```

That is the _entire_ system. It's all stripped down to these few programs, configuration files, and one symlink for DNS resolution. This is a very minimal system, and it's all you need to run statically linked Go programs. It's very easy to deploy your own services to it too. It's probably the easiest platform I know of that lets you just deploy a Go binary and have it run as a service, automatically restarting when it crashes.

## The tooling

When I used gokrazy back in the day, you had to use a command line called `gokr-packer` that you passed a bunch of command line flags to with information about all the Go programs you wanted to run on the machine, configuration for those programs, and any other meta-information like where the update tool should push the image to. It was a bit of a pain to use, but it worked. Recently the [`gok`](https://gokrazy.org/quickstart/) tool was added to the project, and this has been _revolutionary_ when it comes to using and administrating gokrazy installs.

Essentially, `gok` is a wrapper around the existing `gokr-packer` logic with a JSON file to store your configuration details. It's a lot easier to use, understand, and automate. You don't have to remember command line flags or maintain unwieldy scripts. You just edit a JSON file and push updates with `gok update`. It's amazingly simple.

## Setting up a gokrazy machine

As an example, I'm going to show you how to install a bunch of tailnet addons to a gokrazy machine. I'm also going to assume that you don't have a gokrazy install set up yet, so we'll need to install it. To do this, we'll need to do a few simple things:

- Install the `gok` tool.
- Create your `gok` configuration.
- Install Tailscale on the machine.
- Create your "seed" image with `gok overwrite`.
- Boot it on your Raspberry Pi or VM.
- Push any updates to the image to the machine with `gok update`.

First, let's install the `gok` tool. In order to do this, you need to have the [Go toolchain](https://golang.org/doc/install) installed. Once you have that, you can run `go install` to install the `gok` tool:

```bash
go install github.com/gokrazy/tools/cmd/gok@main
```

<xeblog-conv name="Mara" mood="hacker" standalone>You may want to ensure that `~/go/bin` is in your `$PATH` variable so that you can run it by the name `gok` instead of `~/go/bin/gok`.</xeblog-conv>

Next, create a new gokrazy configuration with `gok new`:

```bash
gok new -i casa
```

This will create a configuration named `casa` (cf: Spanish for "house") in `~/gokrazy/casa`. This is where all of your configuration files will live. You can edit the configuration file with `gok edit`:

```bash
gok edit -i casa
```

<details>
<summary>If you are making a virtual machine</summary>

If you are making a virtual machine, you will need to override the kernel and firmware packages. You can do this by adding the following to your configuration file:

```json
{
  // ...
  "KernelPackage": "github.com/rtr7/kernel",
  "FirmwarePackage": "github.com/rtr7/kernel"
  // ...
}
```

You will need to prefix the `gok overwrite` and `gok update` commands with `GOARCH=amd64` to ensure that Go builds x86_64 binaries instead of ARM binaries:

```
GOARCH=amd64 gok update -i casa
```

If you don't do this, you will get arm64 binaries being built. This may require manual recovery of your virtual machine.

</details>

Let's make our lives easier by installing [Tailscale](https://tailscale.com) on the machine. By default, gokrazy will announce its hostname over DHCP, which usually makes most consumer routers pick it up and then lets you ping it by name. When you have [MagicDNS](https://tailscale.com/kb/1081/magicdns/) enabled, Tailscale can take over this logic and prevent you from accessing the machine by name.

However, Tailscale is written in Go and doesn't require any of the services that most Linux distributions provide in order to function. It's a perfect fit for gokrazy. You can install it with `gok add`:

```
gok add tailscale.com/cmd/tailscaled
gok add tailscale.com/cmd/tailscale
```

And be sure to add the `mkfs` service to create a persistent partition on `/perm`:

```
gok add github.com/gokrazy/mkfs
```

Next, fetch an [auth key](https://tailscale.com/kb/1085/auth-keys/) from [the admin console](https://login.tailscale.com/admin/settings/keys) and make sure you check that it's reusable. Then, add the following to your configuration file under the `PackageConfig` block:

```json
{
  // ...
  "PackageConfig": {
    // ...
    "tailscale.com/cmd/tailscale": {
      "CommandLineFlags": [
        "up",
        // paste your key here!
        "--authkey=tskey-auth-hunter2-hunter2hunter2hunter2"
      ]
    }
    // ...
  }
  // ...
}
```

<xeblog-conv name="Mara" mood="hacker" standalone>You can pass any other [`tailscale up` flags](https://tailscale.com/kb/1080/cli/#up) you want here, such as `--advertise-exit-node` if you want to use your gokrazy machine as an [exit node](https://tailscale.com/kb/1103/exit-nodes/?q=exit%20node).</xeblog-conv>

This will make your machine automatically connect to Tailscale on boot.

Next, we need to create our "seed" image with `gok overwrite`. First, figure out what the device node for your SD card is. On Linux, you can do this with `lsblk`:

```
lsblk
```

And then look for the one that has the same size as your SD card. In my case, it's `/dev/sdd`. Once you have that, you can run `gok overwrite`:

```
gok overwrite --full /dev/sdd
```

However if you want to write the image to a file (such as if you are doing mass distribution or making a VM image), you need to use `gok overwrite` with a file instead of a device node. This will create a 16 GB image:

```
gok overwrite -i casa --full gokrazy.img --target_storage_bytes 17179869184
```

Once you have your image, you can write it to your SD card with `dd` (or [balenaEtcher](https://etcher.balena.io/)) or import it into your virtual machine hypervisor of choice.

Once you have your image written to your SD card, you can boot it on your Raspberry Pi or VM.

<xeblog-conv name="Aoi" mood="wut">Wait, so how do I log in with a shell?</xeblog-conv>
<xeblog-conv name="Cadey" mood="enby">You don't. gokrazy doesn't have a login prompt. It's a single-user system. There is [`breakglass`](https://github.com/gokrazy/breakglass) as a tool of last resort to modify things, but you only have a very minimal subset of busybox to work with, so it should be avoided if at all possible.</xeblog-conv>

Once you have your machine booted and it responds to pings over Tailscale, you can open its HTTP interface in your browser. If you called your machine `casa`, you can open it at [`http://casa`](http://casa). It will prompt you for a username and password. Your username is `gokrazy`, and the password is near the top of your `config.json` file. When you log in, you'll see a screen like this:

<xeblog-picture path="blog/2023/gokrazy/gokrazy-ui"></xeblog-picture>

This is the gokrazy web UI. It lets you see the status of your machine and any logs that are being generated by your applications. You can also start, stop, and restart any of your applications from here. It's a very simple UI, but it's fantastic for debugging and monitoring.

## Tailnet addons

Now that we have a Gokrazy system up and running, let's add some programs to it! I'm going to list a couple tailnet addons that give your tailnet superpowers. These are all written in Go, so they're a perfect fit for gokrazy.

Today I'm going to show you how to install these tools into your tailnet:

- [golink](https://github.com/tailscale/golink) - a URL shortener at `http://go`
- [tmemes](https://github.com/tailscale/tmemes) - an internal meme generator you can host at `http://memegen`
- [tclip](https://github.com/tailscale-dev/tclip) - a pastebin you can host at `http://paste`

These tools help you augment your tailnet by giving you tools that will make you and your team's life a lot easier. A URL shortener helps you link to complicated Google Docs URLs. A meme generator gives you a new innovative way to let off steam. A pastebin lets you share text with your team without having to worry about the service you're using going offline due to no fault of your own.

### golink

To install golink, we need to add the `golink` binary to the configuration. You can do this with `gok add`:

```
gok add github.com/tailscale/golink/cmd/golink
```

Then configure it with `gok edit`:

```json
{
  // ...
  "PackageConfig": {
    // ...
    "github.com/tailscale/golink/cmd/golink": {
      "CommandLineFlags": ["--sqlitedb=/perm/home/golink/data.db"],
      "Environment": [
        // the same one from before
        "TS_AUTHKEY=tskey-auth-hunter2-hunter2hunter2hunter2"
      ],
      // don't start the service until NTP catches up
      "WaitForClock": true
    }
    // ...
  }
  // ...
}
```

And finally push it with `gok update`:

```
gok update -i casa
```

It'll build the image, push it out over Tailscale, trigger a reboot, and be back up in the span of a minute. Once it's back up, you can open the web UI again and see the status of your `golink` instance at [http://casa/status?path=%2fuser%2fgolink](http://casa/status?path=%2fuser%2fgolink):

<xeblog-picture path="blog/2023/gokrazy/golink"></xeblog-picture>

And then you can start using short URLs at [http://go](http://go):

<xeblog-picture path="blog/2023/gokrazy/golink-ui"></xeblog-picture>

And that's it! You now have a super minimal VM running small programs that let you do useful things to you. You can add more programs to your configuration file and push them with `gok update` to add more functionality to your machine. You can even add your own programs to the configuration file and push them to your machine. It's a very simple system, but it's very powerful.

### tmemes

Google is infamous for having an internal service named [memegen](https://www.buzzfeednews.com/article/reyhan/inside-googles-internal-meme-generator). This allows Googlers to make internal-facing memes about the slings and arrows that impact them as highly paid programmers. This is an internal service inside Google that has a lot of serious investment of time and energy to make it the best possible experience it can be. It's to the point that reportedly people can keep up with how an all-hands meeting is going by the tone of the sarcastic memes that are being posted to memegen.

The main reason this is run inside Google is to avoid information leaking via memes. Yes, this is an actual threat model.

Thanks to the magic of Tailscale, you can make your own private memegen using [tmemes](https://github.com/tailscale/tmemes). tmemes is a tailnet addon that lets you post image macro templates and layer wisdom over it in the form of text.

Here's an example meme:

<xeblog-picture path="blog/2023/gokrazy/society-if-gokrazy"></xeblog-picture>

To add tmemes to your gokrazy machine, you can use `gok add`:

```
gok add github.com/tailscale/tmemes/tmemes
```

Then open your config with `gok edit` and add the following to your `PackageConfig` block:

```json
{
  // ...
  "PackageConfig": {
    // ...
    "github.com/tailscale/tmemes/tmemes": {
      "Environment": ["TS_AUTHKEY=tskey-auth-hunter2-hunter2hunter2hunter2"],
      "CommandLineFlags": [
        // change this to your desired hostname
        "--hostname=memegen",
        // change this to your username on Tailscale
        "--admin=Xe@github",
        "--store=/perm/home/tmemes"
      ],
      "WaitForClock": true
    }
    // ...
  }
  // ...
}
```

And then push it with `gok update`:

```
gok update -i casa
```

Then you can head to [http://memegen](http://memegen) and upload a template to make your own dank memes.

If you want to integrate your own tools with tmemes, you can check out the [API documentation](https://github.com/tailscale/tmemes/blob/main/docs/api.md). This should help you do whatever it is you want with a meme generator as a service.

### tclip

Sometimes you just need a place to paste text and get a URL pointing to it. [tclip](https://tailscale.dev/blog/tclip) is a tool that you can add to your tailnet and get exactly that. It's a very simple tool, but it's very useful. It's also written in Go, so it's a perfect fit for gokrazy. [Their recent update to remove Cgo dependencies](https://tailscale.dev/blog/tclip-updates-092023) makes it possible to run your tclip node on a gokrazy machine.

To add tclip to your gokrazy machine, you can use `gok add`:

```
gok add github.com/tailscale-dev/tclip/cmd/tclipd
```

Then open your config with `gok edit` and add the following to your `PackageConfig` block:

```json
{
  // ...
  "PackageConfig": {
    // ...
    "github.com/tailscale-dev/tclip/cmd/tclipd": {
      "CommandLineFlags": ["--data-location=/perm/home/tclip/"],
      "WaitForClock": true,
      "Environment": [
        "TS_AUTHKEY=tskey-auth-hunter2-hunter2hunter2hunter2",
        "USE_FUNNEL=true" // Remove this if you don't want to use Funnel
      ]
    }
    // ...
  }
}
```

And then push it with `gok update`:

```
gok update -i casa
```

And then you can start using it by heading to [http://paste](http://paste). Install the command-line tool on your development workstation with `go install`:

```
go install github.com/tailscale-dev/tclip/cmd/tclip@latest
```

Here's an example tclip link if you want to see what it looks like in practice: [interjection.c](https://paste.shark-harmonic.ts.net/paste/696b9b02-90ac-4adc-a33d-d749bb6f460f). It's a very simple tool, but it's very useful.

## Conclusion

gokrazy is insanely cool. It's the _easiest_ way to deploy Go services to your homelab. It integrates seamlessly with Tailscale, and is something that I'm very excited to see grow and mature. I'm very excited to see what the future holds for gokrazy, and I'm very excited to see what people do with it.

I've seen signs that they're going to be adding an automatic update process, and that has me _very_ excited. I'm also excited to see what other services people add to the gokrazy ecosystem. I'm hoping to add a few of my own in the future, and I'm hoping to see what other people do with it.

<xeblog-conv name="Mara" mood="hacker" standalone>Spoiler alert: [waifud support](https://github.com/Xe/waifud-gok-agent) is coming soon to a homelab near you.</xeblog-conv>
