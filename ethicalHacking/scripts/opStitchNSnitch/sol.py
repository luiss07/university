from pwn import *
import zlib

#p = remote('cyberchallenge.disi.unitn.it', 8030)

#p.recvuntil(b'file:\n')

# hex_file = p.recvline().strip().decode()
# binary_data = binascii.unhexlify(hex_file)
# decompressed_data = zlib.decompress(binary_data)

# with open('bin', 'wb') as f:
#     f.write(decompressed_data)

#print(hex_file)

elf = ELF('./bin')
context.arch = 'amd64'

# List symbols at program
# for key, address in elf.symbols.items():
#     print(key, hex(address))

# ---- Change local_10 value from 0x0 to 0x1000 (enter immediately in the if condition) ----
mov_local_10 = 0x1232
local_10_new_value = 0x1000
elf.asm(mov_local_10, 'mov dword ptr [RBP + -0x8], %s' % local_10_new_value)

# ---- Change the puts("Good luck!") to puts(DAT_xxxxxx) ----

# 1. find the offset added to DAT_00104060 to get the flag DAT_xxxxxx (changes every time)

offset_addr = 0x1214
offset_instruction = elf.read(offset_addr, 7)
offset_value = '0x'+offset_instruction[::-1].hex()[:8]
print(offset_value)

# 2. calculate the new address of the flag

dat_base_addr = '0x104060'
flag_addr = int(dat_base_addr,16) + int(offset_value,16)
print(flag_addr, type(flag_addr))

# 3. change the puts("Good luck!") to puts(DAT_xxxxxx)

# change the lea instruction to the new flag address

puts_addr = 0x1282
elf.asm(puts_addr, f'lea RAX, [rip + {flag_addr}]')
elf.save('./patchedBin')

# original_instruction_bytes = elf.read(local_10, 3)
# print(original_instruction_bytes)
# new_instruction_bytes = original_instruction_bytes + new_value

# # for key, address in elf.bss().items():
# #     print(key, hex(address))


# string_addr = 0x1282
# original_string_bytes = elf.read(string_addr, 3)
# print(original_string_bytes)
# new_string_bytes = original_string_bytes + flag_addr

# with open('./bin', 'r+b') as f:
#     # Seek to the address of the original instruction
#     f.seek(local_10)

#     # Write the new instruction bytes
#     f.write(new_instruction_bytes)

#print(elf.symbols[p64(local_10)])

