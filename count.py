from pathlib import Path
import sys


def count(f: Path, data: dict):
    for line in f.open():
        try:
            name_a, name_b, score_a, score_b = line.split()
        except ValueError:
            continue
        names = (name_a, name_b)
        if names not in data:
            data[names] = {}
        scores = (int(score_a), int(score_b))
        data[names][scores] = data[names].get(scores, 0)  + 1


def main():
    data = {}
    for f in sys.argv[1:]:
        f = Path(f)
        count(f, data)
