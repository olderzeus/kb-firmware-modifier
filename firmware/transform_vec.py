#!/usr/bin/env python3
import re
import sys
from pathlib import Path


def transform_nested_vec_text(text: str) -> str:
    # 内側の vec![ ... ] を順に処理する
    inner_vec_pattern = re.compile(r"vec!\s*\[(.*?)\]", re.DOTALL)

    def replace_inner_vec(match: re.Match) -> str:
        body = match.group(1)

        # 整数を抽出
        nums = [int(x) for x in re.findall(r"-?\d+", body)]

        # V[i][j] -> V[i][j] + j
        transformed = [v + j for j, v in enumerate(nums)]

        # 整形して出力
        lines = []
        width = 6
        per_line = 12

        for k in range(0, len(transformed), per_line):
            chunk = transformed[k:k + per_line]
            line = " " * 12 + "".join(f"{n:>{width}d}," for n in chunk)
            lines.append(line)

        return "vec![\n" + "\n".join(lines) + "\n        ]"

    return inner_vec_pattern.sub(replace_inner_vec, text)


def main():
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} input.txt", file=sys.stderr)
        sys.exit(1)

    input_path = Path(sys.argv[1])
    text = input_path.read_text(encoding="utf-8")

    output = transform_nested_vec_text(text)
    print(output)


if __name__ == "__main__":
    main()