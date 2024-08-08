# ビルド

```sh
$ less Makefile
vagrant@ubuntu-bionic:~/ruxpin$ make create-image
dd if=/dev/zero of=ruxpin-ext2-image.bin bs=4K count=1048576
1048576+0 records in
1048576+0 records out
4294967296 bytes (4.3 GB, 4.0 GiB) copied, 8.55915 s, 502 MB/s
dd if=partition-table.bin of=ruxpin-ext2-image.bin bs=512 count=1 conv=notrunc
1+0 records in
1+0 records out
512 bytes copied, 0.0204762 s, 25.0 kB/s
sudo losetup --offset 272629760	 /dev/loop8 ruxpin-ext2-image.bin
losetup: ruxpin-ext2-image.bin: failed to set up loop device: Device or resource busy
make: *** [Makefile:23: create-image] Error 1
$ vi Makefile
-LOOPBACK = /dev/loop8
+LOOPBACK = /dev/loop1

$ make create-image
dd if=/dev/zero of=ruxpin-ext2-image.bin bs=4K count=1048576
1048576+0 records in
1048576+0 records out
4294967296 bytes (4.3 GB, 4.0 GiB) copied, 8.0386 s, 534 MB/s
dd if=partition-table.bin of=ruxpin-ext2-image.bin bs=512 count=1 conv=notrunc
1+0 records in
1+0 records out
512 bytes copied, 0.00044918 s, 1.1 MB/s
sudo losetup --offset 272629760	 /dev/loop1 ruxpin-ext2-image.bin
sudo mkfs.ext2 -b 4096 /dev/loop1 982016
mke2fs 1.46.5 (30-Dec-2021)
Discarding device blocks: done
Creating filesystem with 982016 4k blocks and 245760 inodes
Filesystem UUID: a9644b56-aa97-479d-a423-a01036f9113f
Superblock backups stored on blocks:
	32768, 98304, 163840, 229376, 294912, 819200, 884736

Allocating group tables: done
Writing inode tables: done
Writing superblocks and filesystem accounting information: done

sudo losetup -d /dev/loop1

$ make load-image
make mount-image
make[1]: Entering directory '/home/vagrant/ruxpin'
sudo losetup --offset 272629760	 /dev/loop1 ruxpin-ext2-image.bin
sudo mount -t ext2 /dev/loop1 build
mount: build: mount point does not exist.
make[1]: *** [Makefile:29: mount-image] Error 32
make[1]: Leaving directory '/home/vagrant/ruxpin'
make: *** [Makefile:36: load-image] Error 2

$ mkdir build
vagrant@ubuntu-bionic:~/ruxpin$ make load-image
make mount-image
make[1]: Entering directory '/home/vagrant/ruxpin'
sudo losetup --offset 272629760	 /dev/loop1 ruxpin-ext2-image.bin
losetup: ruxpin-ext2-image.bin: failed to set up loop device: Device or resource busy
make[1]: *** [Makefile:28: mount-image] Error 1
make[1]: Leaving directory '/home/vagrant/ruxpin'
make: *** [Makefile:36: load-image] Error 2
vagrant@ubuntu-bionic:~/ruxpin$ sudo losetup -d /dev/loop1
vagrant@ubuntu-bionic:~/ruxpin$ make load-image
make mount-image
make[1]: Entering directory '/home/vagrant/ruxpin'
sudo losetup --offset 272629760	 /dev/loop1 ruxpin-ext2-image.bin
sudo mount -t ext2 /dev/loop1 build
make[1]: Leaving directory '/home/vagrant/ruxpin'
make load-image-contents
make[1]: Entering directory '/home/vagrant/ruxpin'
cd bin/sh && cargo build --release && rust-strip target/aarch64-unknown-none/release/sh
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: latest update on 2024-07-21, rust version 1.81.0-nightly (506985649 2024-07-20)
info: downloading component 'rust-std' for 'aarch64-unknown-none'
info: installing component 'rust-std' for 'aarch64-unknown-none'
    Updating crates.io index
  Downloaded proc-macro2 v1.0.39
  Downloaded unicode-ident v1.0.0
  Downloaded quote v1.0.18
  Downloaded syn v1.0.95
  Downloaded 4 crates (341.5 KB) in 0.40s
   Compiling proc-macro2 v1.0.39
   Compiling unicode-ident v1.0.0
   Compiling syn v1.0.95
   Compiling ruxpin_types v0.1.0 (/home/vagrant/ruxpin/lib/types)
   Compiling ruxpin_syscall v0.1.0 (/home/vagrant/ruxpin/lib/syscall)
   Compiling quote v1.0.18
   Compiling ruxpin_syscall_proc v0.1.0 (/home/vagrant/ruxpin/lib/syscall_proc)
   Compiling ruxpin_api v0.1.0 (/home/vagrant/ruxpin/lib/api)
   Compiling ruxpin_app v0.1.0 (/home/vagrant/ruxpin/lib/app)
warning: variable does not need to be mutable
  --> /home/vagrant/ruxpin/lib/app/src/allocator.rs:44:13
   |
44 |         let mut nextfree: *mut Block;
   |             ----^^^^^^^^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
   --> /home/vagrant/ruxpin/lib/app/src/allocator.rs:107:13
    |
107 |         let mut block: *mut Block = ptr.cast::<Block>().offset(-1);
    |             ----^^^^^
    |             |
    |             help: remove this `mut`

warning: `ruxpin_app` (lib) generated 2 warnings (run `cargo fix --lib -p ruxpin_app` to apply 2 suggestions)
   Compiling sh v0.1.0 (/home/vagrant/ruxpin/bin/sh)
    Finished `release` profile [optimized] target(s) in 6.50s
error: `llvm-tools-preview` component is missing or empty. Install it with `rustup component add llvm-tools-preview`
cd bin/coreutils && cargo build --release && cd ../../ && rust-strip  bin/coreutils/target/aarch64-unknown-none/release/ls  bin/coreutils/target/aarch64-unknown-none/release/args  bin/coreutils/target/aarch64-unknown-none/release/cat  bin/coreutils/target/aarch64-unknown-none/release/ps  bin/coreutils/target/aarch64-unknown-none/release/rm  bin/coreutils/target/aarch64-unknown-none/release/mv  bin/coreutils/target/aarch64-unknown-none/release/mkdir  bin/coreutils/target/aarch64-unknown-none/release/echo  bin/coreutils/target/aarch64-unknown-none/release/sync
   Compiling proc-macro2 v1.0.39
   Compiling unicode-ident v1.0.0
   Compiling syn v1.0.95
   Compiling ruxpin_types v0.1.0 (/home/vagrant/ruxpin/lib/types)
   Compiling ruxpin_syscall v0.1.0 (/home/vagrant/ruxpin/lib/syscall)
   Compiling quote v1.0.18
   Compiling ruxpin_syscall_proc v0.1.0 (/home/vagrant/ruxpin/lib/syscall_proc)
   Compiling ruxpin_api v0.1.0 (/home/vagrant/ruxpin/lib/api)
   Compiling ruxpin_app v0.1.0 (/home/vagrant/ruxpin/lib/app)
warning: variable does not need to be mutable
  --> /home/vagrant/ruxpin/lib/app/src/allocator.rs:44:13
   |
44 |         let mut nextfree: *mut Block;
   |             ----^^^^^^^^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
   --> /home/vagrant/ruxpin/lib/app/src/allocator.rs:107:13
    |
107 |         let mut block: *mut Block = ptr.cast::<Block>().offset(-1);
    |             ----^^^^^
    |             |
    |             help: remove this `mut`

warning: `ruxpin_app` (lib) generated 2 warnings (run `cargo fix --lib -p ruxpin_app` to apply 2 suggestions)
   Compiling coreutils v0.1.0 (/home/vagrant/ruxpin/bin/coreutils)
    Finished `release` profile [optimized] target(s) in 6.12s
error: `llvm-tools-preview` component is missing or empty. Install it with `rustup component add llvm-tools-preview`
sudo mkdir -p build/bin
sudo cp bin/sh/target/aarch64-unknown-none/release/sh build/bin
sudo cp  bin/coreutils/target/aarch64-unknown-none/release/ls  bin/coreutils/target/aarch64-unknown-none/release/args  bin/coreutils/target/aarch64-unknown-none/release/cat  bin/coreutils/target/aarch64-unknown-none/release/ps  bin/coreutils/target/aarch64-unknown-none/release/rm  bin/coreutils/target/aarch64-unknown-none/release/mv  bin/coreutils/target/aarch64-unknown-none/release/mkdir  bin/coreutils/target/aarch64-unknown-none/release/echo  bin/coreutils/target/aarch64-unknown-none/release/sync build/bin
make[1]: Leaving directory '/home/vagrant/ruxpin'
make umount-image
make[1]: Entering directory '/home/vagrant/ruxpin'
sudo umount build
sudo losetup -d /dev/loop1
make[1]: Leaving directory '/home/vagrant/ruxpin'

$ cd config/raspberrypi3/
$ ls
build.rs  Cargo.lock  Cargo.toml  chainboot.sh  Makefile  qemu.sh  src
$ make
ARCH=aarch64 cargo build --release
    Updating crates.io index
  Downloaded proc-macro2 v1.0.38
  Downloaded unicode-xid v0.2.3
  Downloaded syn v1.0.92
  Downloaded 3 crates (293.5 KB) in 0.19s
   Compiling proc-macro2 v1.0.38
   Compiling unicode-xid v0.2.3
   Compiling syn v1.0.92
   Compiling ruxpin_types v0.1.0 (/home/vagrant/ruxpin/lib/types)
   Compiling ruxpin_syscall v0.1.0 (/home/vagrant/ruxpin/lib/syscall)
   Compiling ruxpin_config_raspberrypi3 v0.1.0 (/home/vagrant/ruxpin/config/raspberrypi3)
   Compiling quote v1.0.18
   Compiling ruxpin_syscall_proc v0.1.0 (/home/vagrant/ruxpin/lib/syscall_proc)
   Compiling ruxpin_kernel v0.1.0 (/home/vagrant/ruxpin/kernel)
   Compiling ruxpin_api v0.1.0 (/home/vagrant/ruxpin/lib/api)
warning: creating a mutable reference to mutable static is discouraged
  --> /home/vagrant/ruxpin/kernel/src/mm/pages.rs:48:9
   |
48 |         &mut PAGES
   |         ^^^^^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see issue #114447 <https://github.com/rust-lang/rust/issues/114447>
   = note: this will be a hard error in the 2024 edition
   = note: this mutable reference has lifetime `'static`, but if the static gets accessed (read or written) by any other means, or any other reference is created, then any further use of this mutable reference is Undefined Behavior
   = note: `#[warn(static_mut_refs)]` on by default
help: use `addr_of_mut!` instead to create a raw pointer
   |
48 |         addr_of_mut!(PAGES)
   |         ~~~~~~~~~~~~~     +

warning: unused variable: `path`
  --> /home/vagrant/ruxpin/kernel/src/api/file.rs:94:23
   |
94 | pub fn syscall_getcwd(path: &mut [u8]) -> Result<(), KernelError> {
   |                       ^^^^ help: if this is intentional, prefix it with an underscore: `_path`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: variable does not need to be mutable
  --> /home/vagrant/ruxpin/kernel/src/mm/kmalloc.rs:57:13
   |
57 |         let mut space: *mut Block = start.to_kernel_addr().as_mut();
   |             ----^^^^^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
  --> /home/vagrant/ruxpin/kernel/src/mm/kmalloc.rs:69:13
   |
69 |         let mut nextfree: *mut Block;
   |             ----^^^^^^^^
   |             |
   |             help: remove this `mut`

warning: variable does not need to be mutable
   --> /home/vagrant/ruxpin/kernel/src/mm/kmalloc.rs:110:13
    |
110 |         let mut block: *mut Block = ptr.cast::<Block>().offset(-1);
    |             ----^^^^^
    |             |
    |             help: remove this `mut`

   Compiling ruxpin_filesystems_procfs v0.1.0 (/home/vagrant/ruxpin/filesystems/procfs)
warning: unused variable: `mount`
   --> /home/vagrant/ruxpin/filesystems/procfs/src/lib.rs:176:25
    |
176 |     fs::for_each_mount(|mount| {
    |                         ^^^^^ help: if this is intentional, prefix it with an underscore: `_mount`
    |
    = note: `#[warn(unused_variables)]` on by default

warning: `ruxpin_filesystems_procfs` (lib) generated 1 warning
   Compiling ruxpin_filesystems_ext2 v0.1.0 (/home/vagrant/ruxpin/filesystems/ext2)
warning: variable does not need to be mutable
   --> /home/vagrant/ruxpin/filesystems/ext2/src/directories.rs:179:29
    |
179 | ...   let mut previous_entry_on_disk: &mut Ext2DirEntryHeader = unsafe {
    |           ----^^^^^^^^^^^^^^^^^^^^^^
    |           |
    |           help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default

warning: `ruxpin_filesystems_ext2` (lib) generated 1 warning (run `cargo fix --lib -p ruxpin_filesystems_ext2` to apply 1 suggestion)
   Compiling ruxpin_drivers_arm v0.1.0 (/home/vagrant/ruxpin/drivers/arm)
   Compiling ruxpin_filesystems_devfs v0.1.0 (/home/vagrant/ruxpin/filesystems/devfs)
   Compiling ruxpin_filesystems_tmpfs v0.1.0 (/home/vagrant/ruxpin/filesystems/tmpfs)
   Compiling ruxpin_drivers_raspberrypi v0.1.0 (/home/vagrant/ruxpin/drivers/raspberrypi)
warning: `ruxpin_kernel` (lib) generated 5 warnings (run `cargo fix --lib -p ruxpin_kernel` to apply 3 suggestions)
    Finished `release` profile [optimized] target(s) in 11.58s
rust-objcopy --strip-all -O binary target/aarch64-unknown-none/release/ruxpin ruxpin.img
error: `llvm-tools-preview` component is missing or empty. Install it with `rustup component add llvm-tools-preview`

$ rustup component add llvm-tools-preview
info: downloading component 'llvm-tools'
info: installing component 'llvm-tools'
 31.3 MiB /  31.3 MiB (100 %)  12.3 MiB/s in  2s ETA:  0s
$ make
ARCH=aarch64 cargo build --release
warning: creating a mutable reference to mutable static is discouraged
  --> /home/vagrant/ruxpin/kernel/src/mm/pages.rs:48:9
   |
48 |         &mut PAGES
   |         ^^^^^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see issue #114447 <https://github.com/rust-lang/rust/issues/114447>
   = note: this will be a hard error in the 2024 edition
   = note: this mutable reference has lifetime `'static`, but if the static gets accessed (read or written) by any other means, or any other reference is created, then any further use of this mutable reference is Undefined Behavior
   = note: `#[warn(static_mut_refs)]` on by default
help: use `addr_of_mut!` instead to create a raw pointer
   |
48 |         addr_of_mut!(PAGES)
   |         ~~~~~~~~~~~~~     +

warning: unused variable: `path`
  --> /home/vagrant/ruxpin/kernel/src/api/file.rs:94:23
   |
94 | pub fn syscall_getcwd(path: &mut [u8]) -> Result<(), KernelError> {
   |                       ^^^^ help: if this is intentional, prefix it with an underscore: `_path`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: variable does not need to be mutable
  --> /home/vagrant/ruxpin/kernel/src/mm/kmalloc.rs:57:13
   |
57 |         let mut space: *mut Block = start.to_kernel_addr().as_mut();
   |             ----^^^^^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
  --> /home/vagrant/ruxpin/kernel/src/mm/kmalloc.rs:69:13
   |
69 |         let mut nextfree: *mut Block;
   |             ----^^^^^^^^
   |             |
   |             help: remove this `mut`

warning: variable does not need to be mutable
   --> /home/vagrant/ruxpin/kernel/src/mm/kmalloc.rs:110:13
    |
110 |         let mut block: *mut Block = ptr.cast::<Block>().offset(-1);
    |             ----^^^^^
    |             |
    |             help: remove this `mut`

warning: `ruxpin_kernel` (lib) generated 5 warnings (run `cargo fix --lib -p ruxpin_kernel` to apply 3 suggestions)
warning: unused variable: `mount`
   --> /home/vagrant/ruxpin/filesystems/procfs/src/lib.rs:176:25
    |
176 |     fs::for_each_mount(|mount| {
    |                         ^^^^^ help: if this is intentional, prefix it with an underscore: `_mount`
    |
    = note: `#[warn(unused_variables)]` on by default

warning: `ruxpin_filesystems_procfs` (lib) generated 1 warning
warning: variable does not need to be mutable
   --> /home/vagrant/ruxpin/filesystems/ext2/src/directories.rs:179:29
    |
179 | ...   let mut previous_entry_on_disk: &mut Ext2DirEntryHeader = unsafe {
    |           ----^^^^^^^^^^^^^^^^^^^^^^
    |           |
    |           help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default

warning: `ruxpin_filesystems_ext2` (lib) generated 1 warning (run `cargo fix --lib -p ruxpin_filesystems_ext2` to apply 1 suggestion)
    Finished `release` profile [optimized] target(s) in 0.03s
rust-objcopy --strip-all -O binary target/aarch64-unknown-none/release/ruxpin ruxpin.img

$ ./qemu.sh
gtk initialization failed

$ vi qemu.sh

$ ./qemu.sh
starting kernel...
kernel heap: using 0x200000, size 14MiB
virtual memory: using region at PhysicalAddress(0x1000000), size 240 MiB, pages 61408
using 32 pages (130560 bytes) for descriptors (ratio of 1920)
alloc 7680 1, desc 122880 30
interrupts: initializing generic arm interrupt controller
fs: registering filesystem devfs
fs: registering filesystem procfs
fs: registering filesystem tmpfs
fs: registering filesystem ext2
console: initializing
sd: initializing
# ここでストール
QEMU: Terminated
./qemu.sh: line 14: -drive: command not found
```

## Linux GUIで実行

```sh
$ ./qemu.sh
starting kernel...
kernel heap: using 0x200000, size 14MiB
virtual memory: using region at PhysicalAddress(0x1000000), size 240 MiB, pages 61408
using 32 pages (130560 bytes) for descriptors (ratio of 1920)
alloc 7680 1, desc 122880 30
interrupts: initializing generic arm interrupt controller
fs: registering filesystem devfs
fs: registering filesystem procfs
fs: registering filesystem tmpfs
fs: registering filesystem ext2
console: initializing
sd: initializing
mmc: reading size 0 at offset 0 into buffer.size 400
sd: found partition 0 at 2000, 256 MiB
sd: found partition 1 at 82000, 740 MiB
fs: mounting ext2 at /, device Some(DeviceID(0, 2))
mmc: reading size 2e400000 at offset 400 into buffer.size 400
ext2: magic number ef53, block size 4096
ext2: total blocks 982016, total inodes 245760, unallocated blocks: 964609, unallocated inodes: 245738
ext2: features compat: 38, ro: 3, incompat: 2
mmc: reading size 2e400000 at offset 1000 into buffer.size 1000
mmc: reading size 2e400000 at offset f2000 into buffer.size 1000
mmc: reading size 2e400000 at offset f3000 into buffer.size 1000
mmc: reading size 2e400000 at offset 2f3000 into buffer.size 1000
mmc: reading size 2e400000 at offset f1000 into buffer.size 1000
mmc: reading size 2e400000 at offset 2f9000 into buffer.size 1000
ext2: allocating inode 12
mmc: reading size 2e400000 at offset 2fa000 into buffer.size 1000
ext2: allocating inode 13
mmc: reading size 2e400000 at offset 2fb000 into buffer.size 1000
ext2: allocating inode 14
fs: mounting devfs at /dev, device None
fs: mounting procfs at /proc, device None
fs: mounting tmpfs at /tmp, device None

Running some hardcoded tests before completing the startup

Creating a directory and a file inside of it
mmc: reading size 2e400000 at offset 2fc000 into buffer.size 1000
ext2: allocating inode 15
ext2: allocating inode 16
mmc: reading size 2e400000 at offset 2fd000 into buffer.size 1000
Read file 14: This is a test

Opening the console device file and writing to it
the device file can write

Opening the shell binary through the vfs interface and reading some data
mmc: reading size 0x2e400000 at offset 0x60001000 into buffer.size 1000
Rust Panic: panicked at /home/vagrant/ruxpin/drivers/raspberrypi/src/emmc.rs:76:33:
range end index 18446744072874881024 out of range for slice of length 4096
qemu-system-aarch64: terminating on signal 2
# 18446744072874881024 = 0xFFFFFFFFCE3FF000  : 現在も再現
#                      = -1908412416
# 0x1000 + 0x1000 > 0x2e400000 (740MB)
```

1. これに関係があるか不明だが`todo.txt`に以下の文がある。

      emmcドライバには問題があり、qemuで使用するイメージが2GB以下の場合、Readコマンドは
      バイトオフセットを与えるが、4GB以上の場合はセクタオフセット（バイトオフセット / 512）を
      与える。私は8GBのカードしか持っていないのでこれがpiでも起こるかどうかはわからない。
      カードサイズを検出する方法があれば、解決できるかもしれない。

2. xv6-fudanのメモとソースに以下の記述があった

   - SD Cardタイプが違う。実機はHC, QEMUはSC
   - アドレスはカードタイプにより異なり、HCはブロック番号をアドレスとして
   私、SCはアドレスをそのまま渡す

3. 現在のemmcドライバのオフセット値はアドレスが設定されていると思われる.

   ```rust
   let nbytes = block::raw_read(device_id, &mut *entry.block.lock(), (block_num as usize * block_size) as u64)?;
   ```

4. `qemu_srouce/hw/sd/sd.c`

```c
# include/hw/registerfields.h
#define FIELD(reg, field, shift, length)                                  \
    enum { R_ ## reg ## _ ## field ## _SHIFT = (shift)};                  \
    enum { R_ ## reg ## _ ## field ## _LENGTH = (length)};                \
    enum { R_ ## reg ## _ ## field ## _MASK =                             \
                                        MAKE_64BIT_MASK(shift, length)};

# hw/sd/sd.c
static void sd_response_r3_make(SDState *sd, uint8_t *response)
{
    stl_be_p(response, sd->ocr & ACMD41_R3_MASK);
}
```

## 解決: `partition-table.bin`を変更

- `partition-table.bin`のext2のパーティションの総セクタ数が`0x17200`となっている。
- この場合、容量は`0x17200 * 0x200 = 0x2e4_000 (740MB)`である。
- ext2は`0xefc00 * 0x1000 = 0xefc0_0000 (4GB)`で作成している
- そのため、オフセット`0x60001000`が領域外となっていた。
- `partition-table.bin`の総セクタ数を`0x77e000 = 0xefc0_0000 / 0x200`に変更
- これで上のエラーはなくなった
- ただし、`sh`は動かない

### QEMUの実行ログ

```sh
$ ./qemu.sh 
starting kernel...
kernel heap: using 0x200000, size 14MiB
virtual memory: using region at PhysicalAddress(0x1000000), size 240 MiB, pages 61408
using 32 pages (130560 bytes) for descriptors (ratio of 1920)
alloc 7680 1, desc 122880 30
interrupts: initializing generic arm interrupt controller
fs: registering filesystem devfs
fs: registering filesystem procfs
fs: registering filesystem tmpfs
fs: registering filesystem ext2
console: initializing
sd: initializing
mmc: sending command GoIdle 0
mmc: sending command SendIfCond 1aa
mmc: sending command AppCommand 0
mmc: sending command SendOpCond 51ff8000
mmc: sending command SendCID 0
mmc: sending command SendRelAddr 0
mmc: sending command CardSelect 45670500
sd: found partition 0 at 2000, 256 MiB
sd: found partition 1 at 82000, 3836 MiB
fs: mounting ext2 at /, device Some(DeviceID(0, 2))
mmc: sending command GoIdle 0
mmc: sending command SendIfCond 1aa
mmc: sending command AppCommand 0
mmc: sending command SendOpCond 51ff8000
mmc: sending command SendCID 0
mmc: sending command SendRelAddr 0
mmc: sending command CardSelect 45670500
ext2: magic number ef53, block size 4096
ext2: total blocks 982016, total inodes 245760, unallocated blocks: 964609, unallocated inodes: 245738
ext2: features compat: 38, ro: 3, incompat: 2
ext2: allocating block 761 in group 0
storing inode 12
ext2: allocating inode 12
ext2: allocating block 762 in group 0
storing inode 13
ext2: allocating inode 13
ext2: allocating block 763 in group 0
storing inode 14
ext2: allocating inode 14
fs: mounting devfs at /dev, device None
ext2: looking for "dev", found inode 12
ext2: looking for "..", found inode 2
fs: mounting procfs at /proc, device None
ext2: looking for "proc", found inode 13
ext2: looking for "..", found inode 2
fs: mounting tmpfs at /tmp, device None
ext2: looking for "tmp", found inode 14
ext2: looking for "..", found inode 2

Running some hardcoded tests before completing the startup

Creating a directory and a file inside of it
ext2: allocating block 764 in group 0
storing inode 15
ext2: allocating inode 15
ext2: looking for "testdir", found inode 15
ext2: looking for "testdir", found inode 15
storing inode 16
ext2: allocating inode 16
ext2: allocating block 765 in group 0
ext2: writing to block 765
storing inode 16
Read file 14: This is a test

Opening the console device file and writing to it
ext2: looking for "dev", found inode 12
the device file can write

Opening the shell binary through the vfs interface and reading some data
ext2: looking for "bin", found inode 16385
ext2: looking for "sh", found inode 16386
read in 1024 bytes
0xffff00000007f9c0: 7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00 
0xffff00000007f9d0: 02 00 b7 00 01 00 00 00 10 34 21 00 00 00 00 00 
0xffff00000007f9e0: 40 00 00 00 00 00 00 00 80 77 00 00 00 00 00 00 
0xffff00000007f9f0: 00 00 00 00 40 00 38 00 06 00 40 00 08 00 07 00 
0xffff00000007fa00: 06 00 00 00 04 00 00 00 40 00 00 00 00 00 00 00 
0xffff00000007fa10: 40 00 20 00 00 00 00 00 40 00 20 00 00 00 00 00 
0xffff00000007fa20: 50 01 00 00 00 00 00 00 50 01 00 00 00 00 00 00 
0xffff00000007fa30: 08 00 00 00 00 00 00 00 01 00 00 00 04 00 00 00 
0xffff00000007fa40: 00 00 00 00 00 00 00 00 00 00 20 00 00 00 00 00 
0xffff00000007fa50: 00 00 20 00 00 00 00 00 94 18 00 00 00 00 00 00 
0xffff00000007fa60: 94 18 00 00 00 00 00 00 00 00 01 00 00 00 00 00 
0xffff00000007fa70: 01 00 00 00 05 00 00 00 a0 18 00 00 00 00 00 00 
0xffff00000007fa80: a0 18 21 00 00 00 00 00 a0 18 21 00 00 00 00 00 
0xffff00000007fa90: 70 5e 00 00 00 00 00 00 70 5e 00 00 00 00 00 00 
0xffff00000007faa0: 00 00 01 00 00 00 00 00 01 00 00 00 06 00 00 00 
0xffff00000007fab0: 10 77 00 00 00 00 00 00 10 77 22 00 00 00 00 00 
0xffff00000007fac0: 10 77 22 00 00 00 00 00 30 00 00 00 00 00 00 00 
0xffff00000007fad0: 31 00 00 00 00 00 00 00 00 00 01 00 00 00 00 00 
0xffff00000007fae0: 50 e5 74 64 04 00 00 00 50 18 00 00 00 00 00 00 
0xffff00000007faf0: 50 18 20 00 00 00 00 00 50 18 20 00 00 00 00 00 
0xffff00000007fb00: 14 00 00 00 00 00 00 00 14 00 00 00 00 00 00 00 
0xffff00000007fb10: 04 00 00 00 00 00 00 00 51 e5 74 64 06 00 00 00 
0xffff00000007fb20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
0xffff00000007fb30: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
0xffff00000007fb40: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
0xffff00000007fb50: 53 6f 6d 65 4e 6f 6e 65 20 20 20 20 70 69 64 20 
0xffff00000007fb60: 00 00 00 00 00 00 00 00 08 00 00 00 00 00 00 00 
0xffff00000007fb70: 08 00 00 00 00 00 00 00 a0 18 21 00 00 00 00 00 
0xffff00000007fb80: 00 00 00 00 00 00 00 00 08 00 00 00 00 00 00 00 
0xffff00000007fb90: 08 00 00 00 00 00 00 00 e0 3a 21 00 00 00 00 00 
0xffff00000007fba0: 10 19 21 00 00 00 00 00 f0 19 21 00 00 00 00 00 
0xffff00000007fbb0: b7 00 37 41 23 5a 64 46 6e b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fbc0: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fbd0: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fbe0: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fbf0: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fc00: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fc10: b9 b9 b9 b9 82 3c 87 1e 4b b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fc20: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fc30: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fc40: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fc50: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fc60: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fc70: b9 b9 b9 b9 b9 b9 b9 b9 91 2d 9b 55 73 96 5f 69 
0xffff00000007fc80: 8c a0 32 28 af 0f a5 aa 78 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fc90: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fca0: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fcb0: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fcc0: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 
0xffff00000007fcd0: b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 b9 50 7d 19 14 
0xffff00000007fce0: 05 0a 45 72 72 6f 72 41 64 64 72 65 73 73 41 6c 
0xffff00000007fcf0: 72 65 61 64 79 4d 61 70 70 65 64 41 64 64 72 65 
0xffff00000007fd00: 73 73 55 6e 6d 61 70 70 65 64 41 64 64 72 65 73 
0xffff00000007fd10: 73 4d 69 73 61 6c 69 67 6e 65 64 55 6e 65 78 70 
0xffff00000007fd20: 65 63 74 65 64 47 72 61 6e 75 61 6c 65 53 69 7a 
0xffff00000007fd30: 65 43 6f 72 72 75 70 74 54 72 61 6e 73 6c 61 74 
0xffff00000007fd40: 69 6f 6e 54 61 62 6c 65 4f 75 74 4f 66 4d 65 6d 
0xffff00000007fd50: 6f 72 79 4e 6f 53 65 67 6d 65 6e 74 46 6f 75 6e 
0xffff00000007fd60: 64 4d 65 6d 6f 72 79 50 65 72 6d 69 73 73 69 6f 
0xffff00000007fd70: 6e 44 65 6e 69 65 64 4c 6f 63 6b 54 69 6d 65 6f 
0xffff00000007fd80: 75 74 4e 6f 53 75 63 68 44 65 76 69 63 65 4f 70 
0xffff00000007fd90: 65 72 61 74 69 6f 6e 4e 6f 74 50 65 72 6d 69 74 
0xffff00000007fda0: 74 65 64 44 65 76 69 63 65 54 69 6d 65 6f 75 74 
0xffff00000007fdb0: 49 4f 45 72 72 6f 72 49 6e 76 61 6c 69 64 49 72 

Opening a new file and writing some data into it
storing inode 17
ext2: allocating inode 17
ext2: allocating block 766 in group 0
ext2: writing to block 766
storing inode 17

Reading back the data written previously
ext2: looking for "test2", found inode 17
0xffff00000007f9c0: 74 68 69 73 20 69 73 20 73 6f 6d 65 20 74 65 73 
0xffff00000007f9d0: 74 20 64 61 74 61 00 00 00 00 00 00 00 00 00 00 
0xffff00000007f9e0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
0xffff00000007f9f0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
0xffff00000007fa00: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
0xffff00000007fa10: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
0xffff00000007fa20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
0xffff00000007fa30: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 

Printing the contents of the root directory (ext2 mount)
reading dir . with inode 2
reading dir .. with inode 2
reading dir lost+found with inode 11
reading dir bin with inode 16385
reading dir dev with inode 12
reading dir proc with inode 13
reading dir tmp with inode 14
reading dir testdir with inode 15
reading dir test2 with inode 17

Finished tests

loading the first processs (/bin/sh) from elf binary file
ext2: looking for "bin", found inode 16385
ext2: looking for "sh", found inode 16386
ext2: looking for "bin", found inode 16385
ext2: looking for "sh", found inode 16386
program segment 0: 6 4 offset: 40 v:200040 p:200040 size: 150
program segment 1: 1 4 offset: 0 v:200000 p:200000 size: 1894
program segment 2: 1 5 offset: 18a0 v:2118a0 p:2118a0 size: 5e70
program segment 3: 1 6 offset: 7710 v:227710 p:227710 size: 30
program segment 4: 6474e550 4 offset: 1850 v:201850 p:201850 size: 14
program segment 5: 6474e551 6 offset: 0 v:0 p:0 size: 0
ext2: looking for "dev", found inode 12
timer: initializing generic arm timer to trigger context switch
kernel initialization complete
scheduler: starting multitasking
```



```sh
loading the first processs (/bin/sh) from elf binary file
pages: allocating page at 0x1020000
ext2: looking for "bin", found inode 180225
ext2: looking for "sh", found inode 180226
ext2: looking for "bin", found inode 180225
ext2: looking for "sh", found inode 180226
pages: allocating page at 0x1021000
pages: incrementing page ref at 0x1021000
pages: incrementing page ref at 0x1021000
program segment 0: 6 4 offset: 40 v:200040 p:200040 size: 150
program segment 1: 1 4 offset: 0 v:200000 p:200000 size: 1894
pages: allocating page at 0x1022000
pages: allocating page at 0x1023000
pages: allocating page at 0x1024000
program segment 2: 1 5 offset: 18a0 v:2118a0 p:2118a0 size: 5e70
program segment 3: 1 6 offset: 7710 v:227710 p:227710 size: 30
program segment 4: 6474e550 4 offset: 1850 v:201850 p:201850 size: 14
program segment 5: 6474e551 6 offset: 0 v:0 p:0 size: 0
pages: allocating page at 0x1025000
pages: allocating page at 0x1026000
pages: allocating page at 0x1027000
load process /bin/sh (2)
ext2: looking for "dev", found inode 12
timer: initializing generic arm timer to trigger context switch
kernel initialization complete
scheduler: starting multitasking
context swith from 1
context swith to 2
Handle a user exception of ESR: 8200000b from ELR: 213410
Instruction or Data Abort 8200000b caused by Access Flag at address 213410 (allocating new page)
pages: allocating page at 0x1028000
pages: incrementing page ref at 0x1028000
Handle a user exception of ESR: 9200004b from ELR: 213418
Instruction or Data Abort 9200004b caused by Access Flag at address 227710 (allocating new page)
pages: allocating page at 0x1029000
pages: incrementing page ref at 0x1029000
Handle a user exception of ESR: 9200004f from ELR: 213418
Instruction or Data Abort 9200004f caused by Permissions Flag at address 227710 (either copy-on-write or fault)
copying page on write PhysicalAddress(0x1029000)
pages: allocating page at 0x102a000
pages: decrementing page ref at 1029000
Handle a user exception of ESR: 8200000b from ELR: 212380
Instruction or Data Abort 8200000b caused by Access Flag at address 212380 (allocating new page)
pages: allocating page at 0x102b000
pages: incrementing page ref at 0x102b000
Handle a user exception of ESR: 56000001 from ELR: 213af8   # syscall
A SYSCALL for Write!
```

## システムコールが動いていいない?

```sh
$ objdump -d bin/sh/target/aarch64-unknown-none/release/sh
  213ae0:       f9400000        ldr     x0, [x0]      // 引数は1つで何かへのポインタ
  213ae4:       aa1f03e3        mov     x3, xzr
  213ae8:       aa1f03e4        mov     x4, xzr
  213aec:       aa1f03e5        mov     x5, xzr
  213af0:       52800106        mov     w6, #0x8      // #8 = Write
  213af4:       d4000021        svc     #0x1          // suscall
  213af8:       aa0003e8        mov     x8, x0
```

- `sh#main()`の最初の`println`と思われる
- `fs::write()`から戻ってこない
- `PL011::put_char()`のFIFOが開くのを待っていると思われる

```rust
//`in /bin/sh#main()
pub fn main() {
    println!("\nStarting shell...");
}

macro_rules! println {
    ($($args:tt)*) => ({
        use core::fmt::Write;
        $crate::UnbufferedFile::stdout().write_fmt(format_args!($($args)*)).unwrap();
        $crate::UnbufferedFile::stdout().write_str("\n").unwrap();
    })
}

impl Write for UnbufferedFile {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(self.0, s.as_bytes()).unwrap();
        Ok(())
    }
}

#[syscall_function(Write)]
pub fn write(file: FileDesc, buffer: &[u8]) -> Result<usize, ApiError> {}

#[syscall_handler]
pub fn syscall_write(file: FileDesc, buffer: &[u8]) -> Result<usize, KernelError> {
    let file = scheduler::get_current().try_lock()?.files.try_lock()?.get_file(file)?;
    // 現状はこのfs::write()が復帰しない
    fs::write(file, buffer)
}

// kern/fs/vfs.rs
pub fn write(file: File, buffer: &[u8]) -> Result<usize, KernelError> {
    let mut fptr = file.lock();
    let vnode = fptr.vnode.clone();
    let result = vnode.lock().write(&mut *fptr, buffer)?;
    Ok(result)
}

impl VnodeOperations for DevCharDeviceVnode 
    fn write(&mut self, file: &mut FilePointer, buffer: &[u8]) -> Result<usize, KernelError> {
        let nbytes = tty::write(self.device_id, buffer)?;
        file.position += nbytes;
        Ok(nbytes)
    }

impl CharOperations for PL011Device
   fn write(&mut self, buffer: &[u8]) -> Result<usize, KernelError> {
         for byte in buffer {
               put_char(*byte);
         }
         Ok(buffer.len())
      }

/// PL011に1文字書き出し（書き出し完了までブロック）
fn put_char(byte: u8) {
    unsafe {
        while (PL011.get(registers::FLAGS) & PL011_FLAGS_TX_FIFO_FULL) != 0 { }
        PL011.set(registers::DATA, byte as u32);
    }
}
```

- sysin, sysout, syserrは次の関数でセットしている

```rust
// in binaryies/mod.rs
pub fn load_process(cmd: &str) -> Result<(), KernelError> {
   ...
   let file = fs::open(None, "/dev/console0", OpenFlags::ReadWrite, FileAccess::DefaultFile, 0)?;
   locked_files.set_slot(FileDesc(0), file.clone())?;
   locked_files.set_slot(FileDesc(1), file.clone())?;
   locked_files.set_slot(FileDesc(2), file)?;
}
```

- この関数(`load_process`)は以下のようにブートコアの起動時に一度だけ呼ばれる

```sh
_start
   _boot_core
      boot_core_start()
         register_devices()
            binaries::load_process("/bin/sh").unwrap();
```

# rr

```
$ rr record ./qemu.sh 
rr needs /proc/sys/kernel/perf_event_paranoid <= 1, but it is 4.
Change it to 1, or use 'rr record -n' (slow).
Consider putting 'kernel.perf_event_paranoid = 1' in /etc/sysctl.d/10-rr.conf.
See 'man 8 sysctl', 'man 5 sysctl.d' (systemd systems)
and 'man 5 sysctl.conf' (non-systemd systems) for more details.
```

- silicon mac版のubuntuのqemuは未対応

# lldb

```sh
'/Users/zuki/raspi_os/ruxpin/config/raspberrypi3/qemu.sh' doesn't contain any 'host' platform architectures: arm64, armv7, armv7f, armv7k, armv7s, armv7m, armv7em, armv6m, armv6, armv5, armv4, arm, thumbv7, thumbv7k, thumbv7s, thumbv7f, thumbv7m, thumbv7em, thumbv6m, thumbv6, thumbv5, thumbv4t, thumb, x86_64, x86_64, arm64, arm64e, arm64, arm64e
```

## 各種デバッグ出力を1箇所除いて削除した結果、出力されるようになったが

1. `kernel/src/fs/vfs.rs#write(file: File, buffer: &[u8])`で次のようにデバッグ出力する

   - OK: `notice!("{:?}", buffer);`

2. デバッグ行を削除したり、単なる文字列出力だと`Starting shell...`を出力した後フリーズ

   - NG: `notice!(" ");`
   - NG: この行を削除

3. 以下、[数値]がデバッグ出力。数値は10進の文字コード。実際の出力文字の前にデバッグ出力している

```sh
$ ./qemu.sh 
[10, 83, 116, 97, 114, 116, 105, 110, 103, 32, 115, 104, 101, 108, 108, 46, 46, 46]

Starting shell...
[10]


[10, 37, 32]

% ls

[46]
.
[10]


[46, 46]
..
[10]


[108, 111, 115, 116, 43, 102, 111, 117, 110, 100]
lost+found
[10]


[98, 105, 110]
bin
[10]


[100, 101, 118]
dev
[10]


[112, 114, 111, 99]
proc
[10]


[116, 109, 112]
tmp
[10]


[116, 101, 115, 116, 100, 105, 114]
testdir
[10]


[116, 101, 115, 116, 50]
test2
[10]

Exiting process 3

[112, 105, 100, 32]
pid 
[51]
3
[32, 101, 120, 105, 116, 101, 100, 32, 119, 105, 116, 104, 32]
 exited with 
[48]
0
[10]


[10, 37, 32]

% ls /

[46]
.
[10]


[46, 46]
..
[10]


[108, 111, 115, 116, 43, 102, 111, 117, 110, 100]
lost+found
[10]


[98, 105, 110]
bin
[10]


[100, 101, 118]
dev
[10]


[112, 114, 111, 99]
proc
[10]


[116, 109, 112]
tmp
[10]


[116, 101, 115, 116, 100, 105, 114]
testdir
[10]


[116, 101, 115, 116, 50]
test2
[10]

Exiting process 4

[112, 105, 100, 32]
pid 
[52]
4
[32, 101, 120, 105, 116, 101, 100, 32, 119, 105, 116, 104, 32]
 exited with 
[48]
0
[10]


[10, 37, 32]

% ls /bin

[46]
.
[10]


[46, 46]
..
[10]


[115, 104]
sh
[10]


[108, 115]
ls
[10]


[97, 114, 103, 115]
args
[10]


[99, 97, 116]
cat
[10]


[112, 115]
ps
[10]


[114, 109]
rm
[10]


[109, 118]
mv
[10]


[109, 107, 100, 105, 114]
mkdir
[10]


[101, 99, 104, 111]
echo
[10]


[115, 121, 110, 99]
sync
[10]

Exiting process 5

[112, 105, 100, 32]
pid 
[53]
5
[32, 101, 120, 105, 116, 101, 100, 32, 119, 105, 116, 104, 32]
 exited with 
[48]
0
[10]


[10, 37, 32]

% ls /dev

[82, 117, 115, 116, 32, 80, 97, 110, 105, 99, 58, 32]
Rust Panic: 
[112, 97, 110, 105, 99, 107, 101, 100, 32, 97, 116, 32]
panicked at 
[115, 114, 99, 47, 98, 105, 110, 47, 108, 115, 46, 114, 115]
src/bin/ls.rs
[58]
:
[50, 48]
20
[58]
:
[52, 55]
47
[58, 10]
:

[99, 97, 108, 108, 101, 100, 32, 96, 82, 101, 115, 117, 108, 116, 58, 58, 117, 110, 119, 114, 97, 112, 40, 41, 96, 32, 111, 110, 32, 97, 110, 32, 96, 69, 114, 114, 96, 32, 118, 97, 108, 117, 101]
called `Result::unwrap()` on an `Err` value
[58, 32]
: 
[79, 112, 101, 114, 97, 116, 105, 111, 110, 78, 111, 116, 80, 101, 114, 109, 105, 116, 116, 101, 100]
OperationNotPermitted
[10]

Exiting process 6

[112, 105, 100, 32]
pid 
[54]
6
[32, 101, 120, 105, 116, 101, 100, 32, 119, 105, 116, 104, 32]
 exited with 
[45]
-
[49]
1
[10]


[10, 37, 32]

% ls /proc

[46]
.
[10]


[46, 46]
..
[10]


[49]
1
[10]


[50]
2
[10]


[55]
7
[10]


[109, 111, 117, 110, 116, 115]
mounts
[10]

Exiting process 7

[112, 105, 100, 32]
pid 
[55]
7
[32, 101, 120, 105, 116, 101, 100, 32, 119, 105, 116, 104, 32]
 exited with 
[48]
0
[10]


[10, 37, 32]

%
```
