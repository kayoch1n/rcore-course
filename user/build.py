#!/bin/usr/python3
import os


base_address = 0x80400000
step = 0x200000

with open("src/linker.ld") as f:
    linker_lines = f.readlines()
    
def overwrite_linker(lines):
    with open("src/linker.ld", "w") as f:
        f.writelines(lines)

for app_id, filename in enumerate(sorted(os.listdir("src/bin"))):
    app_name = filename.split('.', 1)[0]
    new_base_address = base_address + step * app_id
    new_linker_lines = [line.replace(hex(base_address), hex(new_base_address)) for line in linker_lines]
    overwrite_linker(new_linker_lines)
    print(f"building {app_name}")
    os.system(f"cargo build --bin {app_name} --release")

overwrite_linker(linker_lines)
