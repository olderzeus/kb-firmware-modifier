import re

def convert_file(input_path, output_path):
    with open(input_path, "r", encoding="utf-8") as f:
        lines = f.readlines()

    block_index = 1  # n（ブロック番号）
    line_index = 0   # ml（ブロック内インデックス）

    output_lines = []

    hex_pattern = re.compile(r"\|\s*(0x[0-9a-fA-F]{4})\s*\|")

    for line in lines:
        stripped = line.strip()

        # 空行 → そのまま出力 + ブロック切り替え
        if stripped == "":
            output_lines.append("")  # 空行維持
            block_index += 1
            line_index = 0
            continue

        # HEX値抽出
        match = hex_pattern.search(line)
        if not match:
            output_lines.append(line.rstrip("\n"))  # 念のためそのまま出力
            continue

        hex_value = match.group(1)

        # tp{n}_{ml}（mlは2桁ゼロ埋め）
        tp_name = f"tp{block_index}_{line_index:02d}"

        # 出力行生成
        output_line = (
            f"    DW           ${{s/{tp_name}/{hex_value}/}}"
            f"   ; | {hex_value} | <- trackpoint accelaration coefficient -| ROM-Loaded POINT (9) <<<"
        )

        output_lines.append(output_line)

        line_index += 1

    with open(output_path, "w", encoding="utf-8") as f:
        f.write("\n".join(output_lines))


# 使用例
convert_file("input.txt", "output.txt")