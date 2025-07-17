import re
import sys
from pathlib import Path
def replace_code_blocks(readme_path="README.md"):
	readme = Path(readme_path)
	content = readme.read_text(encoding="utf-8")
	pattern = re.compile(r"```(\w+):([^\n]+)\n(.*?)```", re.DOTALL)
	def replacer(match):
		lang, filepath, _ = match.groups()
		path = Path(filepath.strip())
		if not path.exists():
			print(f"[WARN] File not found: {filepath}")
			return match.group(0)
		code = path.read_text(encoding="utf-8")
		return f"```{lang}:{filepath}\n{code}```"
	new_content = pattern.sub(replacer, content)
	readme.write_text(new_content, encoding="utf-8")
	print("[OK] README.md updated.")
replace_code_blocks(sys.argv[1])