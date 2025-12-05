import re
import sys
from pathlib import Path
def replace_code_blocks(readme_path="README.md"):
	readme = Path(readme_path)
	content = readme.read_text(encoding="utf-8")
	pattern = re.compile(r"```(\w+) <!--([^\n\s]+)(\s+\d+)?\s*-->\n(.*?)```", re.DOTALL)
	new_content = pattern.sub(replacer, content)
	readme.write_text(new_content, encoding="utf-8")
	print("[OK] README.md updated.")
def replacer(match):
	lang, filepath, line_count, _ = match.groups()
	path = Path(filepath.strip())
	# ファイルが存在しない
	if not path.exists():
		print(f"[WARN] File not found: {filepath}")
		return match.group(0)
	code = path.read_text(encoding="utf-8")
	# 行数制限
	if line_count and line_count.strip():
		code=clamp_text_lines(code, int(line_count.strip()))
	line_count = (line_count or "").strip()
	return f"```{lang} <!--{filepath} {line_count}-->\n{code}\n```"
def clamp_text_lines(text: str, linecount: int, *, sep_width: int = 40) -> str:
	"""
	text を linecount 行以内に収めて返す。
	超過時は、先頭と末尾を同数だけ残し、中央に省略ラインを挿入する。
	- 省略ラインは「─── 省略 N 行 ───」形式。
	- linecount <= 0 なら空文字を返す。
	- linecount == 1 なら省略ラインのみ。
	"""
	if linecount <= 0:
		return ""

	lines = text.splitlines()
	n = len(lines)
	if n <= linecount:
		return text

	# 省略ライン1行を確保して前後同数に分割（“以内”のため偶数時は1行余ることがある）
	keep_each = max((linecount - 1) // 2, 0)  # 前後に残す行数
	head = lines[:keep_each]
	tail = lines[-keep_each:] if keep_each > 0 else []

	omitted = n - (len(head) + len(tail))

	# 省略ライン生成
	bar = "─" * max(sep_width, 12)
	sep = f"{bar} {omitted} lines omitted {bar}"

	out: list[str] = []
	out.extend(head)
	out.append(sep)
	out.extend(tail)
	return "\n".join(out)
if __name__ == "__main__":
	replace_code_blocks(sys.argv[1])