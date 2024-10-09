#!/bin/python3
import sys

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


with open(output_file_path, 'w') as f:
    f.write('#include <stddef.h>\n\n')
    f.write('typedef struct {\n')
    f.write('    size_t address;\n')
    f.write('    const char *name;\n')
    f.write('    size_t size;\n')
    f.write('} Symbol;\n\n')
    f.write('const Symbol SYMBOL_TABLE[] = {\n')

    for line in lines:
        parts = line.split()
        if len(parts) >= 3:
            address = parts[0]
            name = parts[2]
            f.write(f'    {{0x{address}, "{name}", 0}},\n')

    f.write('};\n')
    f.write('const size_t SYMBOL_TABLE_SIZE = sizeof(SYMBOL_TABLE) / sizeof(Symbol);\n')
