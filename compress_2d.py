import argparse


def main():
    p = argparse.ArgumentParser()
    p.add_argument('input', type=argparse.FileType('r'))
    p.add_argument('--width', type=int, default=4)
    args = p.parse_args()

    width = args.width
    f = 1.0 / width / width
    row_count = 0
    col_count = 0
    buf = []
    for line in args.input:
        if not line.strip():
            row_count += 1
            col_count = 0
            if row_count == width:
                row_count = 0
                for x, y, z in buf[:-1]:
                    print(x * f, y * f, z * f)
                print()
                buf = []
            continue
        x, y, z = map(float, line.split())
        index = col_count // width
        if len(buf) <= index:
            buf.append([0.0, 0.0, 0.0])
        buf[index][0] += x
        buf[index][1] += y
        buf[index][2] += z
        col_count += 1


if __name__ == '__main__':
    main()
