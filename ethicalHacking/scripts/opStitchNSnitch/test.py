from pwn import *

# Assemble the instruction in Intel syntax for 64-bit x86
instruction = 'lea rax, [0x1042c6]'
machine_code = asm(instruction, arch='amd64', vma=0x00101282)
bytes = 0x1042c6.to_bytes(3, 'little')
print(machine_code, len(machine_code.hex()), bytes, bytes.hex())