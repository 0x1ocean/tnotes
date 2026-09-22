#!/usr/bin/env python3
"""Regenerates src/assets/fonts/jetbrains-mono-400-700.woff2.

    python3 -m venv .venv && .venv/bin/pip install "fonttools[woff]" brotli
    curl -sL -o /tmp/jb.zip https://github.com/JetBrains/JetBrainsMono/releases/download/v2.304/JetBrainsMono-2.304.zip
    unzip -q /tmp/jb.zip -d /tmp/jb
    .venv/bin/python scripts/subset-font.py "/tmp/jb/fonts/variable/JetBrainsMono[wght].ttf"

Keeps the wght axis but only 400..700 (the site uses 400 and 700), and only
the code points the site renders: ASCII, Latin-1, general punctuation, arrows,
box drawing, block elements, geometric shapes, misc symbols, dingbats.
"""
import sys
from pathlib import Path

from fontTools import subset
from fontTools.ttLib import TTFont
from fontTools.varLib import instancer

OUT = Path(__file__).resolve().parent.parent / "src/assets/fonts/jetbrains-mono-400-700.woff2"
UNICODES = "U+0020-007E,U+00A0-00FF,U+2010-2027,U+2030-203A,U+2190-21FF,U+2500-259F,U+25A0-25FF,U+2600-26FF,U+2700-27BF,U+23CE,U+21E5"

font = instancer.instantiateVariableFont(TTFont(sys.argv[1]), {"wght": (400, 700)})
opts = subset.Options()
opts.layout_features = ["kern", "calt", "liga", "zero"]
opts.notdef_outline = True
subsetter = subset.Subsetter(opts)
subsetter.populate(unicodes=subset.parse_unicodes(UNICODES))
subsetter.subset(font)
font.flavor = "woff2"
font.save(OUT)
print(OUT, OUT.stat().st_size, "bytes")
