#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path


COMMENT_HEX_PATTERN = re.compile(r"\|\s*0x([0-9a-fA-F]{1,4})\s*\|")
DW_HEX_PATTERN = re.compile(r"\bDW\b\s+0x([0-9a-fA-F]{1,4})\b", re.IGNORECASE)


def word_to_signed_i8(hex_digits: str) -> int:
    word = int(hex_digits, 16)
    low8 = word & 0xFF
    return low8 - 256 if low8 >= 0x80 else low8


def extract_value_from_line(line: str) -> int | None:
    m = COMMENT_HEX_PATTERN.search(line)
    if m:
        return word_to_signed_i8(m.group(1))

    m = DW_HEX_PATTERN.search(line)
    if m:
        return word_to_signed_i8(m.group(1))

    return None


def extract_groups(text: str) -> list[list[int]]:
    groups: list[list[int]] = []
    current: list[int] = []

    for line in text.splitlines():
        if line.strip() == "":
            if current:
                groups.append(current)
                current = []
            continue

        v = extract_value_from_line(line)
        if v is not None:
            current.append(v)

    if current:
        groups.append(current)

    return groups


def format_rust_arrays(
    groups: list[list[int]],
    base_name: str = "ACCEL_TABLE",
    per_line: int = 12,
) -> str:
    out_lines: list[str] = []

    for i, values in enumerate(groups, start=1):
        name = f"{base_name}_{i}"
        out_lines.append(f"pub const {name}: [i8; {len(values)}] = [")

        for j in range(0, len(values), per_line):
            chunk = values[j:j + per_line]
            body = ", ".join(f"{v:>4d}" for v in chunk)
            out_lines.append(f"    {body},")

        out_lines.append("];\n")

    return "\n".join(out_lines).rstrip()


def main() -> int:
    if len(sys.argv) < 2:
        print(
            f"Usage: {Path(sys.argv[0]).name} <input.txt> [BASE_NAME]",
            file=sys.stderr,
        )
        return 1

    input_path = Path(sys.argv[1])
    base_name = sys.argv[2] if len(sys.argv) >= 3 else "ACCEL_TABLE"

    try:
        text = input_path.read_text(encoding="utf-8")
    except FileNotFoundError:
        print(f"File not found: {input_path}", file=sys.stderr)
        return 1

    groups = extract_groups(text)
    if not groups:
        print("No table values were found.", file=sys.stderr)
        return 2

    print(format_rust_arrays(groups, base_name))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())