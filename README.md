# ARP based terminal-chat

In case you don't know, ARP is a protocol that allows computers to find each other over a network.

## Limitations

you cannot talk to people on other networks

## things i made terminal-chat do

I have implemented a generalizable transport protocol on top of ARP, which allows you to send messages that are tens of thousands of characters long. I have also added a bit of compression to improve performance.

If you wanted, you could probably split off the networking part of this protocol and use it instead of UDP. However, I do not recommend doing this, as it may not be compatible with all existing networking infrastructure.

In addition to join and leave notifications, I have also built an entire presence discovery and heartbeat system. This system allows you to see an updated list of other online users. Ironically, part of this system serves a similar purpose to ARP itself.

## Running

If you would like to install arpchat, you can download it from the releases page: https://github.com/erdem85/terminal-chat/releases/latest.

On Windows, you will likely need to install npcap: https://npcap.com/#download. Make sure to check the "Install Npcap in WinPcap API-compatible Mode" option in the installer!

On Linux, you may need to grant arpchat network privileges. To do this, open a terminal and run the following command:

```sh
sudo setcap CAP_NET_RAW+ep /path/to/arpchat
```
This will grant arpchat the ability to access the network, which is necessary for it to function properly.

![interface selector](https://doggo.ninja/tvFJ2A.png)

Then run the binary in a terminal. You can verify that it is working properly by sending yourself a message and seeing it appear in the terminal. If you cannot see your messages, try selecting a different interface or protocol.

## Building

I don't recommend building arpchat yourself. However, if you do decide to build it, you will need to use the latest unstable version of Rust.

On Windows, you will also need to download the WinPcap Developer's Pack and set the LIB environment variable to the WpdPack/Lib/x64/ folder.

```sh
cargo build
```

