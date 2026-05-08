#!/usr/bin/env python3
"""
update_changelog.py - Prepends a new entry to CHANGELOG.md.

Usage: python3 update_changelog.py CHANGELOG.md <crate_version> <api_version> <date> [--notes "..."]
"""

import sys
import re
import json
import argparse
from pathlib import Path

HEADER = """# Changelog

All notable changes to **ferobot** are documented here.

Format: [Semantic Versioning](https://semver.org/)  
Auto-generated API updates use the [Telegram Bot API spec](https://github.com/PaulSonOfLars/telegram-bot-api-spec).

---
"""

def load_diff(path="/tmp/diff_report.json"):
    try:
        return json.load(open(path))
    except Exception:
        return None

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("changelog")
    parser.add_argument("crate_ver")
    parser.add_argument("api_ver")
    parser.add_argument("date")
    parser.add_argument("--notes", default="", help="Additional release notes")
    args = parser.parse_args()

    changelog_path = Path(args.changelog)
    diff = load_diff()

    entry_lines = []
    entry_lines.append(f"## [{args.crate_ver}] - {args.date}")
    entry_lines.append("")
    entry_lines.append(f"### Telegram Bot API: `{args.api_ver}`")
    entry_lines.append("")
    entry_lines.append(
        f"**Changelog:** [core.telegram.org/bots/api#recent-changes]"
        f"(https://core.telegram.org/bots/api#recent-changes)"
    )
    entry_lines.append("")

    if args.notes.strip():
        entry_lines.append("**Notes:**")
        for line in args.notes.strip().splitlines():
            entry_lines.append(line)
        entry_lines.append("")

    if diff:
        added_t   = diff.get("added_types", [])
        removed_t = diff.get("removed_types", [])
        added_m   = diff.get("added_methods", [])
        removed_m = diff.get("removed_methods", [])
        changed_t = diff.get("changed_types", {})
        changed_m = diff.get("changed_methods", {})

        if added_t:
            entry_lines.append("**New Types:**")
            for t in added_t:
                entry_lines.append(f"- `{t}`")
            entry_lines.append("")

        if removed_t:
            entry_lines.append("**Removed Types:**")
            for t in removed_t:
                entry_lines.append(f"- ~~`{t}`~~")
            entry_lines.append("")

        if added_m:
            entry_lines.append("**New Methods:**")
            for m in added_m:
                entry_lines.append(f"- `{m}`")
            entry_lines.append("")

        if removed_m:
            entry_lines.append("**Removed Methods:**")
            for m in removed_m:
                entry_lines.append(f"- ~~`{m}`~~")
            entry_lines.append("")

        if changed_t:
            entry_lines.append(f"**Changed Types** ({len(changed_t)}):")
            for type_name, fields in sorted(changed_t.items()):
                entry_lines.append(f"- `{type_name}`")
                for field_name, desc in sorted(fields.items()):
                    entry_lines.append(f"  - `{field_name}`: {desc}")
            entry_lines.append("")

        if changed_m:
            entry_lines.append(f"**Changed Methods** ({len(changed_m)}):")
            for method_name, fields in sorted(changed_m.items()):
                entry_lines.append(f"- `{method_name}`")
                for field_name, desc in sorted(fields.items()):
                    entry_lines.append(f"  - `{field_name}`: {desc}")
            entry_lines.append("")

    elif not args.notes.strip():
        entry_lines.append("Manual release.")
        entry_lines.append("")

    entry_lines.append("---")
    entry_lines.append("")

    new_entry = "\n".join(entry_lines)

    if changelog_path.exists():
        existing = changelog_path.read_text()
        first_section = re.search(r'^## \[', existing, re.MULTILINE)
        if first_section:
            tail = existing[first_section.start():]
            updated = HEADER + "\n" + new_entry + tail
        else:
            updated = HEADER + "\n" + new_entry
    else:
        updated = HEADER + "\n" + new_entry

    changelog_path.write_text(updated)

    with open("/tmp/release_notes.md", "w") as f:
        f.write(f"## ferobot v{args.crate_ver}\n\n")
        f.write(new_entry)

    print(f"✅ CHANGELOG.md updated for v{args.crate_ver}")

if __name__ == "__main__":
    main()
