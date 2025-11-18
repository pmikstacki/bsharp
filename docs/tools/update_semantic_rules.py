#!/usr/bin/env python3
import argparse
import pathlib
import re
from typing import Dict, List, Tuple, Optional, Set

TABLE_HEADER = "| CS Code | Title | Meaning | B# Code | B# Name | Status |"
TABLE_DIV = "| --- | --- | --- | --- | --- | --- |"

Row = Tuple[str, str, str, str, str, str]


def read_text(path: pathlib.Path) -> str:
    return path.read_text(encoding="utf-8")


def write_text(path: pathlib.Path, data: str) -> None:
    path.write_text(data, encoding="utf-8")


def parse_semantic_table(text: str) -> Tuple[List[str], List[Row]]:
    lines = text.splitlines()
    out_lines: List[str] = []
    rows: List[Row] = []
    in_table = False
    for line in lines:
        if not in_table:
            out_lines.append(line)
            if line.strip() == TABLE_HEADER:
                in_table = True
            continue
        # in_table: expect divider then rows until blank or non-pipe
        if line.strip() == TABLE_DIV:
            out_lines.append(line)
            continue
        if not line.strip().startswith("|") or set(line.strip()) == {"|"}:
            # table ended
            out_lines.append(line)
            in_table = False
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        # Pad/trim to 6 columns
        cells = (cells + [""] * 6)[:6]
        rows.append(tuple(cells)[:] )  # type: ignore
    return out_lines, rows


def format_row(r: Row) -> str:
    return "| " + " | ".join(r) + " |"


def parse_cs_errors_table(md: str) -> List[Tuple[str, str, str]]:
    # Returns list of (CSCode, Title, Meaning)
    # The file has a table with rows like: |[CS0006](url)|Error|Message|
    rows: List[Tuple[str, str, str]] = []
    table_re = re.compile(r"^\|\s*\[?(CS\d{4})\]?\([^)]*\)\s*\|\s*[^|]*\|\s*([^|]*)\|\s*$")
    linkless_re = re.compile(r"^\|\s*(CS\d{4})\s*\|\s*[^|]*\|\s*([^|]*)\|\s*$")
    for line in md.splitlines():
        m = table_re.match(line)
        if not m:
            m = linkless_re.match(line)
        if not m:
            continue
        code = m.group(1).strip()
        meaning = m.group(2).strip()
        # Title: derive a short title from the meaning
        title = derive_title_from_meaning(meaning)
        rows.append((code, title, meaning))
    return rows


def derive_title_from_meaning(meaning: str) -> str:
    # Heuristic: take up to first period or 80 chars, title case keywords
    s = meaning.strip()
    stop = min([i for i in [s.find("."), s.find("--")] if i != -1] + [len(s)])
    s = s[:stop].strip()
    if len(s) > 80:
        s = s[:80].rstrip()
    # Simplify braces placeholders
    s = re.sub(r"'\{\d+\}'", "value", s)
    s = re.sub(r"\{\d+\}", "…", s)
    return s


def build_index(rows: List[Row]) -> Dict[str, int]:
    idx: Dict[str, int] = {}
    for i, r in enumerate(rows):
        cs = r[0].strip()
        if cs:
            idx[cs] = i
    return idx

def load_code_list(path: Optional[pathlib.Path]) -> Set[str]:
    if not path:
        return set()
    if not path.exists():
        return set()
    codes: Set[str] = set()
    for line in read_text(path).splitlines():
        s = line.strip().upper()
        if not s or s.startswith("#"):
            continue
        if s.startswith("CS") and len(s) >= 4:
            codes.add(s)
    return codes


def is_likely_semantic(message: str) -> bool:
    # Conservative heuristics: focus on name lookup, type system, overload resolution,
    # accessibility, member lookup, inheritance/override, modifiers, async/await rules.
    s = message.lower()
    semantic_keywords = [
        "does not exist in the current context",
        "ambiguous",
        "cannot implicitly convert",
        "no implicit conversion",
        "inaccessible",
        "not all code paths return a value",
        "inconsistent accessibility",
        "already contains a definition",
        "already defines a member",
        "cannot override",
        "no suitable method found to override",
        "overload",
        "abstract",
        "virtual",
        "sealed",
        "override",
        "async",
        "interface",
        "base class",
        "derived",
        "type parameter",
        "constraints",
        "nullable",
        "const",
        "readonly",
        "static",
        "operator",
        "member",
        "namespace",
        "using directive",
        "assembly reference",
        "property or indexer",
        "event",
    ]
    syntax_like = [
        "invalid token",
        "expected",
        "must be a",
        "only be used",
        "cannot be after",
        "'#' directives",
        "array with a negative size",  # borderline, treat as non-semantic
        "stackalloc",
    ]
    if any(k in s for k in semantic_keywords) and not any(k in s for k in syntax_like):
        return True
    return False


def filter_cs_rows(
    cs_rows: List[Tuple[str, str, str]],
    starts_with: Optional[str],
    code_range: Optional[str],
    semantic_only: bool,
    include_codes: Set[str],
    exclude_codes: Set[str],
) -> List[Tuple[str, str, str]]:
    def in_range(code: str, rng: str) -> bool:
        try:
            a, b = [s.strip().upper() for s in rng.split("-", 1)]
            if not (a.startswith("CS") and b.startswith("CS")):
                return True
            na, nb = int(a[2:]), int(b[2:])
            n = int(code[2:])
            return na <= n <= nb
        except Exception:
            return True

    out: List[Tuple[str, str, str]] = []
    for code, title, meaning in cs_rows:
        if starts_with and not code.startswith(starts_with.upper()):
            continue
        if code_range and not in_range(code, code_range):
            continue
        if code in exclude_codes:
            continue
        if semantic_only and (code not in include_codes) and (not is_likely_semantic(meaning)):
            continue
        out.append((code, title, meaning))
    return out


def merge_rows(existing: List[Row], cs_rows: List[Tuple[str, str, str]], fill_only: bool) -> List[Row]:
    index = build_index(existing)
    merged = existing[:]
    for code, title, meaning in cs_rows:
        if code in index:
            i = index[code]
            cs_code, t, m, bcode, bname, status = merged[i]
            if not t:
                t = title
            if not m:
                m = meaning
            merged[i] = (cs_code or code, t, m, bcode, bname, status)
        elif not fill_only:
            merged.append((code, title, meaning, "", "", ""))
    return merged


def reconstruct_doc(prefix_lines: List[str], rows: List[Row]) -> str:
    out: List[str] = []
    header_emitted = False
    for line in prefix_lines:
        out.append(line)
        if line.strip() == TABLE_HEADER:
            header_emitted = True
    if header_emitted:
        # ensure divider present just after header
        if not out or out[-1].strip() != TABLE_DIV:
            out.append(TABLE_DIV)
        # emit table rows
        for r in rows:
            out.append(format_row(r))
    return "\n".join(out).rstrip() + "\n"


def main() -> None:
    ap = argparse.ArgumentParser(description="Update semantic-rules.md table with CS entries parsed from CSharpErrorsAndWarnings.md")
    ap.add_argument("--root", type=pathlib.Path, default=pathlib.Path(__file__).resolve().parents[2], help="Project root (default: repo root)")
    ap.add_argument("--fill-only", action="store_true", help="Only fill missing Title/Meaning for CS codes that already exist in semantic-rules.md. Do not append new CS rows.")
    ap.add_argument("--starts-with", type=str, default=None, help="Only include CS codes starting with this prefix, e.g. 'CS01'.")
    ap.add_argument("--range", dest="code_range", type=str, default=None, help="Only include CS codes within range, e.g. 'CS0100-CS0199'.")
    ap.add_argument("--semantic-only", action="store_true", help="Only include diagnostics likely to be semantic (conservative heuristics).")
    ap.add_argument("--include-file", type=pathlib.Path, default=None, help="Path to a file listing CS codes to force-include (one per line).")
    ap.add_argument("--exclude-file", type=pathlib.Path, default=None, help="Path to a file listing CS codes to exclude (one per line).")
    ap.add_argument("--dry-run", action="store_true", help="Do not write files; print a unified diff-like summary of changes")
    args = ap.parse_args()

    sem_path = args.root / "docs" / "development" / "semantic-rules.md"
    cs_path = args.root / "docs" / "development" / "CSharpErrorsAndWarnings.md"

    sem_text = read_text(sem_path)
    cs_text = read_text(cs_path)

    prefix_lines, sem_rows = parse_semantic_table(sem_text)
    cs_rows_all = parse_cs_errors_table(cs_text)
    include_codes = load_code_list(args.include_file)
    exclude_codes = load_code_list(args.exclude_file)
    cs_rows = filter_cs_rows(
        cs_rows_all,
        args.starts_with,
        args.code_range,
        args.semantic_only,
        include_codes,
        exclude_codes,
    )

    merged_rows = merge_rows(sem_rows, cs_rows, fill_only=args.fill_only)
    new_doc = reconstruct_doc(prefix_lines, merged_rows)

    if new_doc == sem_text:
        print("No changes.")
        return

    if args.dry_run:
        # Minimal diff: show added/changed lines in table
        old_lines = sem_text.splitlines()
        new_lines = new_doc.splitlines()
        import difflib
        diff = difflib.unified_diff(old_lines, new_lines, fromfile=str(sem_path), tofile=str(sem_path), lineterm="")
        for line in diff:
            print(line)
        return

    write_text(sem_path, new_doc)
    print(f"Updated {sem_path}")


if __name__ == "__main__":
    main()
