# RUXPINディレクトリ構成

```bash
.
├── LICENSE
├── Makefile
├── README.md
├── bin
│   ├── coreutils
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── bin
│   │           ├── args.rs
│   │           ├── cat.rs
│   │           ├── echo.rs
│   │           ├── ls.rs
│   │           ├── mkdir.rs
│   │           ├── mv.rs
│   │           ├── ps.rs
│   │           ├── rm.rs
│   │           └── sync.rs
│   └── sh
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── config
│   └── raspberrypi3
│       ├── Cargo.lock
│       ├── Cargo.toml
│       ├── Makefile
│       ├── build.rs
│       ├── chainboot.sh
│       ├── qemu.sh
│       └── src
│           └── main.rs
├── drivers
│   ├── arm
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── gic.rs
│   │       ├── lib.rs
│   │       └── timer.rs
│   └── raspberrypi
│       ├── Cargo.toml
│       └── src
│           ├── console.rs
│           ├── emmc.rs
│           └── lib.rs
├── filesystems
│   ├── devfs
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── ext2
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── blocks.rs
│   │       ├── directories.rs
│   │       ├── files.rs
│   │       ├── inodes.rs
│   │       ├── lib.rs
│   │       ├── mount.rs
│   │       └── superblock.rs
│   ├── procfs
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   └── tmpfs
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
├── kernel
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       ├── api
│       │   ├── binaries
│       │   │   ├── elf
│       │   │   │   ├── defs.rs
│       │   │   │   ├── loader.rs
│       │   │   │   └── mod.rs
│       │   │   └── mod.rs
│       │   ├── file.rs
│       │   ├── mod.rs
│       │   └── proc.rs
│       ├── arch
│       │   ├── aarch64
│       │   │   ├── context.rs
│       │   │   ├── exceptions.rs
│       │   │   ├── exceptions.s
│       │   │   ├── kernel.ld
│       │   │   ├── mmu.rs
│       │   │   ├── mmu.s
│       │   │   ├── mod.rs
│       │   │   ├── start.s
│       │   │   └── types.rs
│       │   └── mod.rs
│       ├── block
│       │   ├── bufcache.rs
│       │   ├── mod.rs
│       │   └── partition.rs
│       ├── errors.rs
│       ├── fs
│       │   ├── filedesc.rs
│       │   ├── generic.rs
│       │   ├── mod.rs
│       │   ├── types.rs
│       │   └── vfs.rs
│       ├── irqs.rs
│       ├── lib.rs
│       ├── misc
│       │   ├── byteorder.rs
│       │   ├── cache.rs
│       │   ├── deviceio.rs
│       │   ├── linkedlist.rs
│       │   ├── memory.rs
│       │   ├── mod.rs
│       │   ├── queue.rs
│       │   ├── strarray.rs
│       │   └── writer.rs
│       ├── mm
│       │   ├── kmalloc.rs
│       │   ├── mod.rs
│       │   ├── pagecache.rs
│       │   ├── pages.rs
│       │   ├── segments.rs
│       │   └── vmalloc.rs
│       ├── printk.rs
│       ├── proc
│       │   ├── mod.rs
│       │   ├── scheduler.rs
│       │   └── tasks.rs
│       ├── sync.rs
│       ├── tasklets.rs
│       └── tty
│           ├── canonical.rs
│           └── mod.rs
├── lib
│   ├── api
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── app
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── allocator.rs
│   │       ├── application.ld
│   │       ├── env.rs
│   │       └── lib.rs
│   ├── syscall
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── arch.rs
│   │       └── lib.rs
│   ├── syscall_proc
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   └── types
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
├── partition-table.bin
├── rust-toolchain.toml
├── ruxpin-ext2-image.bin
└── todo.txt
```
