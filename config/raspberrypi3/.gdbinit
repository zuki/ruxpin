set architecture aarch64
file target/aarch64-unknown-none/release/ruxpin
target remote localhost:1234
set print pretty on
set logging off
set height 0

break proc.c:235

#break usb/standardhub.c:220

# Modify the following path to support pwndbg
#source /mnt/d/Workspace/github/pwndbg/gdbinit.py
