# ARP based terminal-chat

In case you don't know, ARP is a protocol that allows computers to find each other over a network.

## limitations

you cannot talk to people on other networks

## things i made terminal-chat do

you can send messages tens of thousands of characters long because i implemented a (naive) generalizable transport protocol on top of arp. there's also a bit of compression.

if you wanted, you could probably split off the networking part of this and use it instead of udp. please don't do this.

not only are join and leave notifications a thing, i built an entire presence discovery and heartbeat system to see an updated list of other online users. ironically, part of this serves a similar purpose to arp itself.


## running

if you actually want to install this for some reason, you can get it from [the releases page](https://github.com/erdem85/terminal-chat/releases/latest).

on windows, you probably need [npcap](https://npcap.com/#download). make sure you check "Install Npcap in WinPcap API-compatible Mode" in the installer!

on linux, you might have to give arpchat network privileges:

```sh
sudo setcap CAP_NET_RAW+ep /path/to/arpchat
```

![interface selector](https://doggo.ninja/tvFJ2A.png)

then just run the binary in a terminal. you know it's working properly if you can see your own messages when you send them. if you *can't* see your messages, try selecting a different interface or protocol!

have any issues? that really sucks. you can make an issue if it pleases you.

## building

you don't really want to build this. anyway, it's tested on the latest unstable rust.

on windows, download the [WinPcap Developer's Pack](https://www.winpcap.org/devel.htm) and set the `LIB` environment variable to the `WpdPack/Lib/x64/` folder.

```sh
cargo build
```

