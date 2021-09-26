scancode_to_colemak = {
    0x16: '1',
    0x1e: '2',
    0x26: '3',
    0x25: '4',
    0x2e: '5',
    0x36: '6',
    0x3d: '7',
    0x3e: '8',
    0x46: '9',
    0x45: '0',
    0x15: 'q',
    0x1d: 'w',
    0x24: 'f',
    0x2d: 'p',
    0x2c: 'g',
    0x35: 'j',
    0x3c: 'l',
    0x43: 'u',
    0x44: 'y',
    0x4d: ';',
    0x1c: 'a',
    0x1b: 'r',
    0x23: 's',
    0x29: ' ',
    0x2b: 't',
    0x34: 'd',
    0x33: 'h',
    0x3b: 'n',
    0x42: 'e',
    0x4b: 'i',
    0x4c: 'o',
    0x1a: 'z',
    0x22: 'x',
    0x21: 'c',
    0x2a: 'v',
    0x32: 'b',
    0x31: 'k',
    0x3a: 'm',
    0x41: ',',
    0x49: '.',
    0x4a: '/',
    0x5a: '\n',
    0x0d: '\t',
    0x66: '\b',
}


def main():
    print(max(scancode_to_colemak.keys()))
    output_list = []
    for i in range(256):
        if i in scancode_to_colemak:
            output_list.append(ord(scancode_to_colemak[i]))
        else:
            output_list.append(0)
    print(output_list)

    print("/*")
    print(" * Generated by generate_keyboard_map.py")
    print(" */")
    print()
    print("#include \"kb_driver.h\"")
    print()
    print("scancode_layout_t colemak = ", end='')
    print("{{", end='')
    print(", ".join([f'{hex(x)}' for x in output_list]), end='')
    print("}};")
    print()
    print("#define ARROW_UP     0x75")
    print("#define ARROW_DOWN   0x72")
    print("#define ARROW_LEFT   0x6B")
    print("#define ARROW_RIGHT  0x74")
    print()
    print("#define SHIFT_LEFT   0x12")
    print("#define SHIFT_RIGHT  0x59")
    print()
    print("#define ESCAPE  0x76")
    print("#define LEFT_CONTROL  0x14")
    print()
    print("#define LEFT_COMMAND  0x1f")
    print("#define LEFT_OPTION  0x11")
    print()


if __name__ == '__main__':
    main()
