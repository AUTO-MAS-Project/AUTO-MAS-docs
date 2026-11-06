Place an appropriately licensed TTF or OTF Unicode font in this folder before
running package.ps1 when the portable package must export Chinese PDFs without
prompting the user to select a font.

Recommended names:
- NotoSansCJKsc-Regular.otf
- NotoSansSC-Regular.ttf

DocForge checks the portable fonts folder first. The package script copies
files from this folder into docforge-portable/fonts.
