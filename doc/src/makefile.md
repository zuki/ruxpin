# create-image

1. 4GiBの `ruxpin-ext2-image.bin` を作成
2. `ruxpin-ext2-image.bin`の先頭に`partition-table.bin`をコピー (MBR: 512B)
3. `ruxpin-ext2-image.bin` のext2領域をloopbackデバイスとして作成
4. ext2ファイルシステムを`BKSIZE=4096`で作成
5. loopbackデバイスを削除

# load-image

1. `mout-image`
    1. ext2 fsをloopbackデバイスとして作成
    2. loopbackデバイスをext2 fsとして`build`ディレクトリにマウント
2. `load-image-contents`
    1. `mkdir build/bin`
    2. `sh`を`build/bin`にコピー
    3. coreutilsを`build/bin`にコピー
3. `loopbackデバイスをアンマウント
