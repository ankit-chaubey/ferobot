# Configuration file for the Sphinx documentation builder.
# https://www.sphinx-doc.org/en/master/usage/configuration.html

import os

# -- Project information -------------------------------------------------------

project   = "ferobot"
copyright = '2026, <a href="https://github.com/ankit-chaubey" target="_blank" rel="noopener">Ankit Chaubey</a>'
author    = "Ankit Chaubey"
release   = "0.1.0"
version   = "0.1"

# -- General configuration -----------------------------------------------------

extensions = [
    "myst_parser",           # Markdown support alongside RST
    "sphinx_copybutton",     # Copy button on all code blocks
    "sphinx.ext.autosectionlabel",  # Auto-label sections for cross-references
]

# MyST settings (Markdown)
myst_enable_extensions = [
    "colon_fence",
    "deflist",
]

# Avoid duplicate label warnings from autosectionlabel across files
autosectionlabel_prefix_document = True

templates_path   = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

source_suffix = {
    ".rst": "restructuredtext",
    ".md":  "markdown",
}

# -- HTML output ---------------------------------------------------------------

html_theme = "furo"

html_theme_options = {
    "sidebar_hide_name": False,
    "navigation_with_keys": True,
    "top_of_page_button": "edit",
    "source_repository": "https://github.com/ankit-chaubey/ferobot",
    "source_branch": "main",
    "source_directory": "docs/",
    "light_css_variables": {
        "color-brand-primary":    "#e8622a",
        "color-brand-content":    "#e8622a",
        "color-admonition-title": "#e8622a",
        "font-stack":             "'IBM Plex Sans', sans-serif",
        "font-stack--monospace":  "'JetBrains Mono', monospace",
    },
    "dark_css_variables": {
        "color-brand-primary":    "#f07a46",
        "color-brand-content":    "#f07a46",
        "color-admonition-title": "#f07a46",
        "font-stack":             "'IBM Plex Sans', sans-serif",
        "font-stack--monospace":  "'JetBrains Mono', monospace",
    },
    "footer_icons": [
        {
            "name": "GitHub",
            "url":  "https://github.com/ankit-chaubey/ferobot",
            "html": (
                '<svg stroke="currentColor" fill="currentColor" stroke-width="0" '
                'viewBox="0 0 16 16" height="1em" width="1em" '
                'xmlns="http://www.w3.org/2000/svg"><path d="M8 0C3.58 0 0 3.58 0 8c0 '
                '3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01'
                '.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53'
                '.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07'
                '-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08'
                '-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53'
                '-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 '
                '3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 '
                '0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z">'
                "</path></svg>"
            ),
            "class": "",
        },
        {
            "name": "crates.io",
            "url":  "https://crates.io/crates/ferobot",
            "html": "📦",
            "class": "",
        },
    ],
}

html_title        = f"ferobot v{release}"
html_short_title  = "ferobot"
html_favicon      = None

# Google Fonts via CSS override
html_css_files = [
    "https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap",
]

html_static_path = ["_static"]
html_extra_path   = ["_static/CNAME"]

# -- Copybutton ----------------------------------------------------------------

copybutton_prompt_text       = r"^\$ "   # strip leading $ from shell snippets
copybutton_prompt_is_regexp  = True
