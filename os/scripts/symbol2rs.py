#!/bin/python3
import sys

def parse_address(address_str):
    return int(address_str, 16)

if len(sys.argv) < 3:
    print("Usage: python3 script.py <path_to_symbols_file> <output_file_path>")
    sys.exit(1)

symbols_file_path = sys.argv[1]
output_file_path = sys.argv[2]

try:
    with open(symbols_file_path, 'r') as f:
        lines = f.readlines()
except FileNotFoundError:
    print(f"Error: File {symbols_file_path} not found.")
    sys.exit(1)

symbols = []

for line in lines:
    parts = line.split()
    if len(parts) >= 3:
        address = parts[0]
        name = parts[2]
        if not name.startswith(".L__unnamed_"):
            symbols.append((address, name))

# Sort symbols by address
symbols.sort(key=lambda x: parse_address(x[0]))

with open(output_file_path, 'w') as f:
    f.write('#![allow(dead_code)]\n')
    f.write('#![allow(unused_variables)]\n')
    f.write('\n')
    f.write('#[repr(C)]\n')
    f.write('pub struct Symbol {\n')
    f.write('    pub address: usize,\n')
    f.write('    pub name: &\'static str,\n')
    f.write('    pub size: usize,\n')
    f.write('}\n\n')
    f.write(f'pub const SYMBOL_TABLE: [Symbol; {len(symbols)}] = [\n')

    for address, name in symbols:
        f.write(f'    Symbol {{ address: 0x{address}, name: "{name}", size: 0 }},\n')

    f.write('];\n')

print(f"Successfully generated Rust symbols file at {output_file_path}.")
