#!/usr/bin/env python3
"""
ferobot RST Reference Generator
Usage:  python3 scripts/generate_rst.py
Output: docs/reference/methods.rst
        docs/reference/types.rst

Reads api.json from the repo root and emits Sphinx RST reference pages.
Run this whenever api.json is updated (CI does it automatically).
"""

import re, json
from pathlib import Path

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------

ROOT    = Path(__file__).parent.parent
API_JSON = ROOT / "api.json"
OUT_DIR  = ROOT / "docs" / "reference"

OUT_DIR.mkdir(parents=True, exist_ok=True)

# ---------------------------------------------------------------------------
# Load spec
# ---------------------------------------------------------------------------

data       = json.loads(API_JSON.read_text(encoding="utf-8"))
API_METHODS = data["methods"]
API_TYPES   = data["types"]

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def to_snake(name: str) -> str:
    s = re.sub(r"([A-Z][a-z]+)", r"_\1", name)
    s = re.sub(r"([a-z])([A-Z])", r"\1_\2", s)
    return s.lower().lstrip("_")

def clean(s: str) -> str:
    return (s.replace("\u2014", "-")
             .replace("\u2013", "-")
             .replace("&mdash;", "-")
             .replace("&ndash;", "-")
             .strip())

def rst_escape(s: str) -> str:
    """Escape RST special chars that would break inline markup."""
    return s.replace("\\", "\\\\").replace("`", "\\`").replace("*", "\\*")

def underline(title: str, char: str) -> str:
    return f"{title}\n{char * len(title)}"

def rst_table(rows: list[dict]) -> str:
    """
    Emit a .. list-table:: for method/type fields.
    rows: list of dicts with keys: name, type, required, description
    """
    if not rows:
        return "No parameters.\n"

    lines = [
        ".. list-table::",
        "   :header-rows: 1",
        "   :widths: 25 20 10 45",
        "",
        "   * - Parameter",
        "     - Type",
        "     - Required",
        "     - Description",
    ]
    for r in rows:
        name  = rst_escape(str(r.get("name", "")))
        # api.json uses a 'types' list, e.g. ["String"] or ["Integer", "String"]
        raw_types = r.get("types", r.get("type", ""))
        if isinstance(raw_types, list):
            typ = " | ".join(raw_types)
        else:
            typ = str(raw_types)
        typ   = rst_escape(typ)
        req   = "Yes" if r.get("required") else "Optional"
        desc_raw = r.get("description", "")
        desc  = clean(" ".join(desc_raw) if isinstance(desc_raw, list) else str(desc_raw))
        desc  = rst_escape(desc[:300])
        lines += [
            f"   * - ``{name}``",
            f"     - ``{typ}``",
            f"     - {req}",
            f"     - {desc}",
        ]
    return "\n".join(lines) + "\n"

# ---------------------------------------------------------------------------
# Method categories (mirrors generate_docs.py ordering)
# ---------------------------------------------------------------------------

CATEGORIES = {
    "Sending Messages":     ["sendMessage","sendPhoto","sendVideo","sendAudio","sendDocument",
                             "sendAnimation","sendVoice","sendVideoNote","sendSticker",
                             "sendMediaGroup","sendLocation","sendVenue","sendContact",
                             "sendDice","sendPoll"],
    "Editing & Deleting":   ["editMessageText","editMessageCaption","editMessageMedia",
                             "editMessageReplyMarkup","editMessageLiveLocation",
                             "stopMessageLiveLocation","deleteMessage","deleteMessages",
                             "pinChatMessage","unpinChatMessage","unpinAllChatMessages"],
    "Forwarding & Copying": ["forwardMessage","forwardMessages","copyMessage","copyMessages"],
    "Callbacks & Inline":   ["answerCallbackQuery","answerInlineQuery","answerWebAppQuery",
                             "savePreparedInlineMessage"],
    "Chat Management":      ["getChat","setChatTitle","setChatDescription","setChatPhoto",
                             "deleteChatPhoto","setChatPermissions",
                             "setChatAdministratorCustomTitle","exportChatInviteLink",
                             "createChatInviteLink","editChatInviteLink","revokeChatInviteLink",
                             "approveChatJoinRequest","declineChatJoinRequest","leaveChat",
                             "createForumTopic","editForumTopic","closeForumTopic",
                             "reopenForumTopic","deleteForumTopic","unpinAllForumTopicMessages",
                             "getForumTopicIconStickers","hideGeneralForumTopic",
                             "unhideGeneralForumTopic","unpinAllGeneralForumTopicMessages"],
    "Chat Members":         ["getChatMember","getChatMemberCount","getChatAdministrators",
                             "banChatMember","unbanChatMember","restrictChatMember",
                             "promoteChatMember","banChatSenderChat","unbanChatSenderChat"],
    "Bot & User Info":      ["getMe","logOut","close","getUserProfilePhotos","getUserChatBoosts",
                             "getMyCommands","setMyCommands","deleteMyCommands",
                             "getMyName","setMyName","getMyDescription","setMyDescription",
                             "getMyShortDescription","setMyShortDescription",
                             "getMyDefaultAdministratorRights","setMyDefaultAdministratorRights",
                             "getChatMenuButton","setChatMenuButton"],
    "Webhooks & Updates":   ["getUpdates","setWebhook","deleteWebhook","getWebhookInfo"],
    "Files":                ["getFile","uploadStickerFile"],
    "Stickers":             ["getStickerSet","createNewStickerSet","addStickerToSet",
                             "setStickerPositionInSet","deleteStickerFromSet","setStickerSetTitle",
                             "setStickerSetThumbnail","setStickerMaskPosition","setStickerEmojiList",
                             "setStickerKeywords","deleteStickerSet","replaceStickerInSet",
                             "setCustomEmojiStickerSetThumbnail"],
    "Payments":             ["sendInvoice","createInvoiceLink","answerShippingQuery",
                             "answerPreCheckoutQuery","refundStarPayment","getStarTransactions",
                             "sendGift","getAvailableGifts"],
    "Games":                ["sendGame","setGameScore","getGameHighScores"],
}
_categorized = {m for ms in CATEGORIES.values() for m in ms}

# ---------------------------------------------------------------------------
# Build methods.rst
# ---------------------------------------------------------------------------

def build_methods_rst() -> str:
    total = len(API_METHODS)
    lines = [
        "Bot Methods",
        "===========",
        "",
        f"All **{total} methods** from Telegram Bot API, auto-generated from ``api.json``.",
        "",
        ".. note::",
        "",
        "   Do not edit this file by hand.",
        "   Run ``python3 scripts/generate_rst.py`` to regenerate.",
        "",
    ]

    # Categorised sections
    for cat, method_names in CATEGORIES.items():
        available = [m for m in method_names if m in API_METHODS]
        if not available:
            continue

        lines += ["", underline(cat, "-"), ""]

        for camel in available:
            info  = API_METHODS[camel]
            snake = to_snake(camel)
            desc  = clean(" ".join(info.get("description", [])))
            ret   = ", ".join(info.get("returns", ["?"]))
            url   = info.get("href", "")
            params = info.get("params", []) or info.get("fields", [])

            lines += [
                f".. _{snake}:",
                "",
                underline(f"``{snake}``", "~"),
                "",
            ]
            if url:
                lines += [f"`Telegram docs ↗ <{url}>`_", ""]
            lines += [desc, "", f"**Returns:** ``{rst_escape(ret)}``", ""]
            lines += [rst_table(params), ""]

    # Uncategorised remainder
    rest = [m for m in API_METHODS if m not in _categorized]
    if rest:
        lines += ["", underline("Other", "-"), ""]
        for camel in rest:
            info  = API_METHODS[camel]
            snake = to_snake(camel)
            desc  = clean(" ".join(info.get("description", [])))
            ret   = ", ".join(info.get("returns", ["?"]))
            url   = info.get("href", "")
            params = info.get("params", []) or info.get("fields", [])

            lines += [
                f".. _{snake}:",
                "",
                underline(f"``{snake}``", "~"),
                "",
            ]
            if url:
                lines += [f"`Telegram docs ↗ <{url}>`_", ""]
            lines += [desc, "", f"**Returns:** ``{rst_escape(ret)}``", ""]
            lines += [rst_table(params), ""]

    return "\n".join(lines)

# ---------------------------------------------------------------------------
# Build types.rst
# ---------------------------------------------------------------------------

def build_types_rst() -> str:
    total = len(API_TYPES)
    lines = [
        "Types Reference",
        "===============",
        "",
        f"All **{total} types** from Telegram Bot API, auto-generated from ``api.json``.",
        "",
        ".. note::",
        "",
        "   Do not edit this file by hand.",
        "   Run ``python3 scripts/generate_rst.py`` to regenerate.",
        "",
    ]

    for name in sorted(API_TYPES.keys()):
        info      = API_TYPES[name]
        fields    = info.get("fields", [])
        desc      = clean(" ".join(info.get("description", [])))
        url       = info.get("href", "")
        subtypes  = info.get("subtypes", [])

        anchor = name.lower()
        lines += [
            f".. _{anchor}:",
            "",
            underline(name, "-"),
            "",
        ]
        if url:
            lines += [f"`Telegram docs ↗ <{url}>`_", ""]
        lines += [rst_escape(desc[:500]), ""]

        if subtypes:
            joined = ", ".join(f"``{s}``" for s in subtypes)
            lines += [f"Union type, one of: {joined}", ""]
        elif fields:
            lines += [rst_table(fields), ""]
        else:
            lines += ["No fields.", ""]

    return "\n".join(lines)

# ---------------------------------------------------------------------------
# Write
# ---------------------------------------------------------------------------

def main():
    methods_path = OUT_DIR / "methods.rst"
    types_path   = OUT_DIR / "types.rst"

    methods_rst = build_methods_rst()
    types_rst   = build_types_rst()

    methods_path.write_text(methods_rst, encoding="utf-8")
    types_path.write_text(types_rst,   encoding="utf-8")

    print(f"[ok] {methods_path}  ({len(API_METHODS)} methods)")
    print(f"[ok] {types_path}  ({len(API_TYPES)} types)")

if __name__ == "__main__":
    main()
