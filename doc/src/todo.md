# TODO

* マルチコアをサポートするために、まずper-CPUのメモリ割り当てが必要である。
* USBドライバを作成する（Ethernetモジュールにアクセスできるように）。

* メモリマップ関数が長さではなく、終了アドレスを取るように変更すべきか。
* スタック・ページ・ガードを追加する。ローダがスタックを作成する際と
  `sbrk/adjust`が呼ばれた際に作成される必要がある。
* mmu変換テーブルのテストを書く

* ページでメモリリークしている可能性がある。シェルの終了時に解放されて
  いないすべての割り当てページを表示することができる。
* ページの参照カウントを自動的に行う方法はあるか。実際のページはページ
  テーブルに`PhysicalAddress`として格納されているが、refcountedの値の増減を
  常に行い、カウントをデクリメントすることなく変換テーブルがrefを破棄して
  から、一度だけ物理アドレスから新しいカウントされていないrefを作成すると
  したらどうか。
  ここでの問題はページ記述子のみから（その固定アドレスはわかっているが）
  ページを決定し、PhysicalAddressを決定し、（`Arc<Page>`と`PhysicalAddress`の
  間を）行ったり来たりできるようにする方法が必要だということである。

* コンソールドライバをバッファードI/Oを使用するように切り替える／修正する
  （割り込みはハードウェアFIFOに直接書き込みことを完全に避けることを難しく
  するような方法ではトリガーされない）


## 優先度の高い問題

* 新しく`subsystems/`ディレクトリを作って、`block`、`tty`、`usb`とおそらくは
  `fs`を置くべきか（そしてコンパイルフラグをもつ）
* このようにした場合、`arch/`, `proc/`, `mm/` は依然として`kernel`に置くべきか。
  `api/`は独立させるか、それともサブシステムに移動するか。
  `misc`は`lib/`のどこかに移動すべきか。
* プロセスのオープンファイルのようなものをリンクするか簡単に見つけられるような形で
  タスク構造体の外に格納するようにできないか。そして、そのような依存性を`proc/`から
  削除して、独立できさせることができないか（カーネルのコードはサブシステムのコードを
  参照してはならないが、`lib/`のコードを参照することはできるだろう）。

* 使われなくなったファイルをpagecachedから削除する方法を追加する必要がある。

* bufcacheとページキャッシュを統一する。
* bufcacheの問題とブロックサイズ対ページサイズの問題については、すべてのbufcacheを
  ページサイズになるようにすることもできるが、実際のページのサブスライスを返すことで
  ファイルシステムが要求するブロックサイズを与えることになる（ページサイズ≧最大ブロック
  サイズの場合のみ機能する）

* should you make the kernel heap use pages instead, and if so, where in virtual memory would the heap space be located?
* need to eventually convert kmalloc to use the paging system (with a kernel page flag if necessary) (I can't remember what sparked this?)

* can you replace the process vec with a hashmap?  The issue is iterating over the list for procfs readdir support

* should you split the api into a linux one and a ruxpin-specific one that is very rusty?  If you did, it would make sense to
  make the various api's modular and external to the kernel, in their own crates


TODO:

* (verify) modify a user fatal error so that it just terminates the process instead of kernel panic

* implement api getcwd()
* add networking
* add threading support in api
* think about multicore and what that would mean for everything

* add a stack guard page to each stack segment, with permissions/mapping in the translation table such that it will cause a fault
  instead of crashing into the adjacent segment.  Linux has this adjustable in size I think
* make the elf loader create a data segment if one is not already created as part of the executable (or should we assume the elf will
  always create a data segment even if not used at all).  So that there's always a data segment to grow when more heap is requested

* add mounts to procfs (and make mount command)

* add an events system for processes to wait on (IO blocking, process exit, select/poll, etc)
* fix the blocking/unblocking code (including exits) to use multiple queues and an event system of some sorts to improve the performance of checks

* add a function to libapp to help parse simple command line arguments

* implement vfs::link()
* make methods on File for fs operations (including unlink/rename?), so you don't have to always use vfs::read(file)
* there's a lot of inconsistency between Ext2BlockNumber and BlockNum in ext2 which should be resolved somehow

* fix the ugly stack manipulation used for the command line arguments.  Can you make one set of arguments available to the process and also
  to procfs (via the task record)?

* add more data to procfs

* add arrow key support to canonical input

* add commands: cp, pwd, mount, umount


ISSUES:

* should you rename unmap_range, map_paged_range, reset_page_copy_on_write, etc to range_unmap, range_map_paged, page_reset_copy_on_write

* think about threads.  I think linux makes every system thread a process with optional sharing of memory and file descriptors, but I
  had been thinking of making thredas separate, making them scheduled, and since there could be a current thread and a current process
  tracked separately by their respective managers (first thing in queue), then a page fault wouldn't have to look up the context, and
  a system call that accessed files would have to fetch both (but not find one via the others)

* need to sort out the tty devices. how irqs can access the device object. whether only the tty subsystem or the driver as well, has
  a reference to the devices as well. how the config will create the device object, and where it stores it (or does it assume each
  driver will register and save global references to the devices if needed?  Is there a better way than using so many global references?

* there's an issue with serial input on the hardware, where it wont show up until a certain number of keys are pressed, but it's a bit
  unpredictable.  I think it might be an issue with when the interrupt occurs based on the buffer fullness??  Could be totally wrong

* sort out issues of copying to/from user space
* there is no userspace/kernelspace copying in the api, which could cause a fatal error if the user program doesn't give a valid
  input, so at some point this needs to be added
* does the linux copy user/kernel function do manual page lookups, including triggering page loading?  Or does it do soemthing else?

* emmcドライバには問題があり、qemuで使用するイメージが2GB以下の場合、Readコマンドは
  バイトオフセットを与えるが、4GB以上の場合はセクタオフセット（バイトオフセット / 512）を
  与える。私は8GBのカードしか持っていないのでこれがpiでも起こるかどうかはわからない。
  カードサイズを検出する方法があれば、解決できるかもしれない。

* should the api be an external crate that integrates the public interfaces (only) of the kernel?

* should you move SyscallFunction type to libapi, since the functions are defined there?  Could you use generics for the syscall itself?
  How will that work for saving the syscall in the task/process?  Could you make a separate place to save that data in the api, on a
  per-task basis?


WHAT IFS:

* how could I make it more event-oriented rather than traditional unixy

* what if you added json for procfs's data, to make it easier to parse?  What if you made json and/or unixstd file formats a compile
  time feature flag?  What if procfs (and all the others) where their own crates with their own feature flags, that are tied together
  by a toplevel config crate (ie. the breakup into crates)

* how would you isolate the unix-specific aspects of the api such that you could implement a light unix-to-ruxpin
  shim on the user process side (or even on the kernel side), so that the ruxpin-native api is not constrained by
  unix but is compatible enough to be source compatible.  Things like waitpid, which take a pointer to the location to put the status,
  it would be nice to have a safer means of passing back data
* can you separate the syscalls, maybe even based on permission levels as well as function, and require permission for each api to
  be granted explicitly, so that APIs security-wise are opt-in, kind of like a web api, or like WASI
* I'm leaning a lot more to the idea of splitting up the APIs into different groups with a different svc number for
  each.  The aarch64 stuff can have the svc-to-api decoding because it might need to be platform specific


LOW PRIORITY TODO:

* modify the proc macro for encoding system calls to be a bit cleaner (not have to put the {} at the end, fit it all in one line, etc)
* there is currently no checking for illegal characters in each path component
* fix rename on ext2 to more intelligently rename (if the parents are the same and the direntry can be reuse, then do that)

* it turns out there's a from_le and from_be function for u32 and others, so maybe you can simplify the byteorder stuff using them
* can you use a weak reference of some kind for the mount link, instead of NonNull?

* add a proper timer driver (ie. fix the hackish ARM timer driver)
* add functions to delay by a set number of microseconds (might need to use the internal counter), for use by drivers

* get the app linker script working better (can you align to 4KB instead of 64KB)
* can you make an improvement on DeviceRegisters, or should you just use tock-registers
