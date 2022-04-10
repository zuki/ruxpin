
TARGET = aarch64-unknown-none/release


MOUNTPOINT = build
IMAGE = ruxpin-ext2-image.bin
BLOCKSIZE = 4096
BLOCKS = 1073741824		# 4GB
PARTITION_OFFSET = 272629760	# Partition 2: 0x8200 * 512
LOOPBACK = /dev/loop8


all: build-kernel


create-image:
	dd if=/dev/zero of=$(IMAGE) bs=4K count=$(BLOCKS)
	sudo losetup $(LOOPBACK) $(IMAGE)
	sudo mkfs.ext2 -b $(BLOCKSIZE) $(LOOPBACK) $(BLOCKS)
	sudo losetup -d $(LOOPBACK)

mount-image:
	sudo losetup $(LOOPBACK) $(IMAGE)
	sudo mount -t ext2 -o offset=$(PARTITION_OFFSET) $(LOOPBACK) $(MOUNTPOINT)

umount-image:
	- sudo umount $(MOUNTPOINT)
	sudo losetup -d $(LOOPBACK)

load-image:
	make mount-image
	- make load-image-contents
	make umount-image

load-image-contents:
	cd bin/testapp && cargo build --release
	sudo mkdir -p $(MOUNTPOINT)/bin
	sudo cp bin/testapp/target/$(TARGET)/testapp $(MOUNTPOINT)/bin/testapp


build-kernel:
	cd kernel && make


