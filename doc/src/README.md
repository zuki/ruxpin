RuxpinOS
========

###### *Started March 1, 2022*

Raspberry Pi 3用のOSをRustで作る試みです。時間的な制約から、新しいアイデアを試す
ことよりも動くものを作ることを優先したので、今のところはUnixライクなAPIを持つ
モノリシックなOSですが、将来的には変更するかもしれません。大部分はC言語で書かれた
[Gloworm OS](https://jabberwocky.ca/projects/gloworm/)で行ったことをコピー
しました。

現時点では、オンデマンドのページ割り当てによる仮想メモリをサポートしていますが、
ディスクへのスワップはまだサポートしていません。ext2ファイルシステムをサポートする
仮想ファイルシステムとインメモリファイルシステムを備えています。システムタイマーを
トリガーとするコンテキストスイッチによるマルチプロセスをサポートしていますが、まだ
マルチスレッドはサポートしていません。

現在のところ、コンソールドライバ（ttyサブシステム）とsd/emmcカードドライバ
（ブロックデバイスサブシステム）しかありません。ブロックドライバサブシステムは
ファイルシステムによりディスクから読み込まれたブロックをキャッシュするための
bufcacheを提供します。可変借用されたブロックはダーティとマークされ、次の
ブロックコミットで書き戻されます。ext2の書き込みサポートはまだ十分にテストされて
いないので、今のところコミットは無効になっています。

アプリケーションはRustで記述し、付属のライブラリでコンパイルすることができます。
このライブラリはAarch64の `SVC` 命令を使用してOSへのシステムコールを実行します。
現在は基本的なファイル操作のほか、`exit`、`fork`、`exec`をサポートしています。
OSはcargoが生成したelfバイナリをアプリケーションとして直接ロードし、実行する
ことができます。シンプルなシェルプログラム（初期化後にカーネルによって起動され
ます）と `ls`コマンドが利用できますが、概念実証の段階に過ぎません。

コンパイル
-----------

OSはカーネルといくつかのアプリケーションで構成され、これらはすべて個別に
コンパイルされます。アプリケーションはカーネルの読むことができるext2
パーティションにロードできますが、カーネルは別にロードする必要があるため、
イメージには含まれていません。

通常、ラズベリーパイが起動すると、ファームウェアはmicroSDカード上のFAT
パーティションから`kernel8.img`を探し、`0x80000`番地にロードして実行します。
カーネルが実行されると、他のパーティションをマウントしてルートファイルシステムと
して使用できます。Qemuで実行する場合、カーネルイメージはext2パーティションを含む
ディスクイメージのファイル名とともにコマンドラインで渡されます。

ext2 ディスクイメージをビルドするための Makefile がプロジェクトルートに用意
されています。qemuとハードウェアでコードをテストするのを容易にするために
microSDカードと同じディスクイメージ（つまり、FATパーティションとext2
パーティション）を作成します。qemuを使用する場合、FATパーティションは無視されます。

イメージを作成するにはLinuxコンピュータ上で以下を実行します。

```sh
make create-image
make load-image
```

これは新しい4GBのイメージファイルを作成し、mkfs.ext2を使ってその中に新しい
ファイルシステムを作成し、それをループバックデバイスとして`<project>/build`に
マウントします。その後、アプリケーションをコンパイルしてロードします。また、
ハードウェアが使用するパーティションを複製した、ハードコーディングされた
パーティションテーブルをイメージにコピーします。

イメージが作成されたら、以下でカーネルをコンパイルしてqemuで実行することが
できます。

```sh
cd config/raspberrypi3/
make
./qemu.sh
```

現状、ラズベリーパイで実行するにはUSBシリアルコンソールが必要です。私は
[Rust Raspberry Pi OS Tutorial](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/) の
MiniLoadプログラムを使ってシリアルポート経由でカーネルをチェーンロードしています。
コンパイルしたカーネルイメージ（`ruxpin.img`）を作成したファイルのブート
パーティションに挿入して、ディスクに書き込むことも可能かもしれませんが、まだ
テストはしていません。

出力例
--------------

以下の出力はqemuで実行した際のコンソールからコピーしたものです。何が行われて
いるかを示すデバッグメッセージがたくさんあります。カーネルヒープとページメモリの
セットアップから始まり、ファイルシステムタイプを登録し、デバイスドライバを
初期化し、ext2ルートパーティションをマウントしています。その後、基本的なファイル
システムの機能をチェックするためにいくつかのテストを実行し、最初のプロセス
（シェル）を起動しています。そして、"<>"で挟まれたコマンドを入力すると、`ls`
プログラムが起動し、`/`ディレクトリのファイルとディレクトリのリストを表示し、
終了するとシェルプロンプトに戻っています

```
starting kernel...
kernel heap: using 0x200000, size 14MiB
virtual memory: using region at PhysicalAddress(0x1000000), size 240 MiB, pages 61438
interrupts: initializing generic arm interrupt controller
fs: registering filesystem tmpfs
fs: registering filesystem devfs
fs: registering filesystem ext2
console: initializing
sd: initializing
sd: found partition 0 at 2000, 256 MiB
sd: found partition 1 at 82000, 740 MiB
fs: mounting ext2 at /, device Some(DeviceID(0, 2))
ext2: magic number ef53, block size 4096
ext2: total blocks 982016, total inodes 245760, unallocated blocks: 963991, unallocated inodes: 245742
ext2: features compat: 38, ro: 3, incompat: 2
ext2: allocating inode 13
fs: mounting devfs at /dev, device None
ext2: looking for "dev", found inode 13

Running some hardcoded tests before completing the startup

Mounting the tmpfs filesystem (simple in-memory file system)
ext2: allocating inode 14
fs: mounting tmpfs at /tmp, device None
ext2: looking for "tmp", found inode 14

Creating a directory and a file inside of it
ext2: allocating inode 15
ext2: looking for "testdir", found inode 15
ext2: allocating inode 16
ext2: allocating block 761 in group 0
ext2: allocating block 762 in group 0
ext2: writing to block 762
Read file 14: This is a test

Opening the console device file and writing to it
ext2: looking for "dev", found inode 13
the device file can write

Opening the testapp binary through the vfs interface and reading some data
ext2: looking for "bin", found inode 32769
ext2: looking for "testapp", found inode 32770
read in 1024 bytes
0xffff00000007f790: 7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
0xffff00000007f7a0: 02 00 b7 00 01 00 00 00 70 29 21 00 00 00 00 00
0xffff00000007f7b0: 40 00 00 00 00 00 00 00 d0 c8 0d 00 00 00 00 00
0xffff00000007f7c0: 00 00 00 00 40 00 38 00 04 00 40 00 10 00 0e 00
0xffff00000007f7d0: 06 00 00 00 04 00 00 00 40 00 00 00 00 00 00 00
0xffff00000007f7e0: 40 00 20 00 00 00 00 00 40 00 20 00 00 00 00 00
0xffff00000007f7f0: e0 00 00 00 00 00 00 00 e0 00 00 00 00 00 00 00
0xffff00000007f800: 08 00 00 00 00 00 00 00 01 00 00 00 04 00 00 00
0xffff00000007f810: 00 00 00 00 00 00 00 00 00 00 20 00 00 00 00 00
0xffff00000007f820: 00 00 20 00 00 00 00 00 60 09 00 00 00 00 00 00
0xffff00000007f830: 60 09 00 00 00 00 00 00 00 00 01 00 00 00 00 00
0xffff00000007f840: 01 00 00 00 05 00 00 00 60 09 00 00 00 00 00 00
0xffff00000007f850: 60 09 21 00 00 00 00 00 60 09 21 00 00 00 00 00
0xffff00000007f860: d8 31 00 00 00 00 00 00 d8 31 00 00 00 00 00 00
0xffff00000007f870: 00 00 01 00 00 00 00 00 51 e5 74 64 06 00 00 00
0xffff00000007f880: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0xffff00000007f890: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0xffff00000007f8a0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0xffff00000007f8b0: 63 61 6c 6c 65 64 20 60 52 65 73 75 6c 74 3a 3a
0xffff00000007f8c0: 75 6e 77 72 61 70 28 29 60 20 6f 6e 20 61 6e 20
0xffff00000007f8d0: 60 45 72 72 60 20 76 61 6c 75 65 00 00 00 00 00
0xffff00000007f8e0: 60 09 21 00 00 00 00 00 08 00 00 00 00 00 00 00
0xffff00000007f8f0: 08 00 00 00 00 00 00 00 20 10 21 00 00 00 00 00
0xffff00000007f900: 60 09 21 00 00 00 00 00 00 00 00 00 00 00 00 00
0xffff00000007f910: 01 00 00 00 00 00 00 00 94 28 21 00 00 00 00 00
0xffff00000007f920: 61 20 72 65 61 6c 6c 79 20 63 6f 6f 6c 20 6d 65
0xffff00000007f930: 73 73 61 67 65 20 74 68 61 74 20 49 27 64 20 6c
0xffff00000007f940: 69 6b 65 20 74 6f 20 73 65 65 00 00 00 00 00 00
0xffff00000007f950: 90 01 20 00 00 00 00 00 2a 00 00 00 00 00 00 00
0xffff00000007f960: 73 72 63 2f 6d 61 69 6e 2e 72 73 00 00 00 00 00
0xffff00000007f970: d0 01 20 00 00 00 00 00 0b 00 00 00 00 00 00 00
0xffff00000007f980: 0c 00 00 00 05 00 00 00 0a 2f 6d 6e 74 2f 74 65
0xffff00000007f990: 73 74 32 00 00 00 00 00 d0 01 20 00 00 00 00 00
0xffff00000007f9a0: 0b 00 00 00 00 00 00 00 0e 00 00 00 51 00 00 00
0xffff00000007f9b0: d0 01 20 00 00 00 00 00 0b 00 00 00 00 00 00 00
0xffff00000007f9c0: 10 00 00 00 28 00 00 00 d0 01 20 00 00 00 00 00
0xffff00000007f9d0: 0b 00 00 00 00 00 00 00 11 00 00 00 19 00 00 00
0xffff00000007f9e0: d0 01 20 00 00 00 00 00 0b 00 00 00 00 00 00 00
0xffff00000007f9f0: 11 00 00 00 2a 00 00 00 d0 01 20 00 00 00 00 00
0xffff00000007fa00: 0b 00 00 00 00 00 00 00 12 00 00 00 11 00 00 00
0xffff00000007fa10: d0 01 20 00 00 00 00 00 0b 00 00 00 00 00 00 00
0xffff00000007fa20: 17 00 00 00 38 00 00 00 d0 01 20 00 00 00 00 00
0xffff00000007fa30: 0b 00 00 00 00 00 00 00 1a 00 00 00 10 00 00 00
0xffff00000007fa40: 72 65 61 64 20 69 6e 20 00 00 00 00 20 00 00 00
0xffff00000007fa50: 4e 6f 74 41 46 69 6c 65 d0 01 20 00 00 00 00 00
0xffff00000007fa60: 0b 00 00 00 00 00 00 00 1c 00 00 00 31 00 00 00
0xffff00000007fa70: d0 01 20 00 00 00 00 00 0b 00 00 00 00 00 00 00
0xffff00000007fa80: 1d 00 00 00 31 00 00 00 64 6f 6e 65 00 00 00 00
0xffff00000007fa90: f8 02 20 00 00 00 00 00 04 00 00 00 00 00 00 00
0xffff00000007faa0: d0 01 20 00 00 00 00 00 0b 00 00 00 00 00 00 00
0xffff00000007fab0: 2d 00 00 00 05 00 00 00 65 78 65 63 75 74 69 6e
0xffff00000007fac0: 67 20 73 65 6c 66 00 00 28 03 20 00 00 00 00 00
0xffff00000007fad0: 0e 00 00 00 00 00 00 00 d0 01 20 00 00 00 00 00
0xffff00000007fae0: 0b 00 00 00 00 00 00 00 24 00 00 00 15 00 00 00
0xffff00000007faf0: 46 69 6c 65 53 69 7a 65 54 6f 6f 4c 61 72 67 65
0xffff00000007fb00: 4e 6f 53 75 63 68 46 69 6c 65 73 79 73 74 65 6d
0xffff00000007fb10: 2f 6d 6e 74 2f 62 69 6e 2f 74 65 73 74 61 70 70
0xffff00000007fb20: 54 6f 6f 4d 61 6e 79 46 69 6c 65 73 4f 70 65 6e
0xffff00000007fb30: 72 61 6e 67 65 20 65 6e 64 20 69 6e 64 65 78 20
0xffff00000007fb40: 50 0e 21 00 00 00 00 00 08 00 00 00 00 00 00 00
0xffff00000007fb50: 08 00 00 00 00 00 00 00 90 0f 21 00 00 00 00 00
0xffff00000007fb60: 60 0e 21 00 00 00 00 00 50 0f 21 00 00 00 00 00
0xffff00000007fb70: 00 05 0a 0f 14 19 1e 23 28 2d 32 37 3c 41 46 4b
0xffff00000007fb80: 50 55 5a 5f 64 69 6e 73 78 7d 82 87 8c 91 63 61

Opening a new file and writing some data into it
ext2: allocating inode 17
ext2: allocating block 763 in group 0
ext2: writing to block 763

Reading back the data written previously
ext2: looking for "test2", found inode 17
0xffff00000007fba0: 74 68 69 73 20 69 73 20 73 6f 6d 65 20 74 65 73
0xffff00000007fbb0: 74 20 64 61 74 61 00 00 00 00 00 00 00 00 00 00
0xffff00000007fbc0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0xffff00000007fbd0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0xffff00000007fbe0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0xffff00000007fbf0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0xffff00000007fc00: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0xffff00000007fc10: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00

Printing the contents of the root directory (ext2 mount)
reading dir . with inode 2
reading dir .. with inode 2
reading dir lost+found with inode 11
reading dir bin with inode 32769
reading dir test with inode 12
reading dir dev with inode 13
reading dir tmp with inode 14
reading dir testdir with inode 15
reading dir test2 with inode 17

Finished tests

loading the first processs (/bin/sh) from elf binary file
ext2: looking for "bin", found inode 32769
ext2: looking for "sh", found inode 32772
program segment 0: 6 4 offset: 40 v:200040 p:200040 size: e0
program segment 1: 1 4 offset: 0 v:200000 p:200000 size: 1870
program segment 2: 1 5 offset: 1870 v:211870 p:211870 size: 53c8
program segment 3: 6474e551 6 offset: 0 v:0 p:0 size: 0
ext2: looking for "dev", found inode 13
timer: initializing generic arm timer to trigger context switch
kernel initialization complete
scheduler: starting multitasking
Instruction or Data Abort caused by Access Flag at address 215a70 (allocating new page)
Instruction or Data Abort caused by Access Flag at address fffffff0 (allocating new page)
Instruction or Data Abort caused by Access Flag at address 21337c (allocating new page)
Instruction or Data Abort caused by Access Flag at address 212190 (allocating new page)

Starting shell...
Instruction or Data Abort caused by Access Flag at address 216c34 (allocating new page)
% <typing in ls>
Instruction or Data Abort caused by Access Flag at address 2140e8 (allocating new page)
executing /bin/ls
child pid is 3
clearing old process space
executing a new process
ext2: looking for "bin", found inode 32769
ext2: looking for "ls", found inode 32774
program segment 0: 6 4 offset: 40 v:200040 p:200040 size: e0
program segment 1: 1 4 offset: 0 v:200000 p:200000 size: 730
program segment 2: 1 5 offset: 730 v:210730 p:210730 size: 2cb8
program segment 3: 6474e551 6 offset: 0 v:0 p:0 size: 0
ext2: looking for "dev", found inode 13
Instruction or Data Abort caused by Access Flag at address 212220 (allocating new page)
Instruction or Data Abort caused by Access Flag at address fffffff0 (allocating new page)
ext2: looking for ".", found inode 2
Instruction or Data Abort caused by Access Flag at address 2133e4 (allocating new page)
Instruction or Data Abort caused by Access Flag at address 2110e4 (allocating new page)
.
..
lost+found
bin
test
dev
tmp
testdir
test2
Exiting process 3
%
```
