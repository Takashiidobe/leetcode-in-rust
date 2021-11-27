.PHONY: phony

BOOK_NAME="Leetcode in Rust"
GRAPHVIZ_FILES = $(shell find ./graphviz -name '*.dot')
FIGURES = $(shell find ./figures -name '*.svg')

PANDOCFLAGS =                        \
	--metadata title=$(BOOK_NAME)      \
  --table-of-contents                \
  --from=markdown                    \
  --number-sections                  \
  --indented-code-classes=javascript

HTML_FLAGS =                         \
	--template=./templates/book.html   \
	--self-contained

PDF_FLAGS =                          \
  --pdf-engine=xelatex               \
  -V mainfont="Palatino"             \
  -V documentclass=report            \
  -V papersize=A5                    \
  -V geometry:"top=1cm, bottom=2cm, left=1cm, right=1cm"


html: phony output/book.html

epub: phony output/book.epub

pdf: phony output/book.pdf

docx: phony output/book.docx

figures: phony
	./bin/figures

output/%.pdf: %.md $(FIGURES) Makefile | output figures
	pandoc $< -o $@ $(PDF_FLAGS) $(PANDOCFLAGS)

output/%.epub: %.md $(FIGURES) Makefile | output figures
	pandoc $< -o $@ $(PANDOCFLAGS)

output/%.html: %.md $(FIGURES) Makefile templates/book.html | output figures
	pandoc $< -o $@ $(HTML_FLAGS) $(PANDOCFLAGS)

output/%.docx: %.md $(FIGURES) Makefile | output figures
	pandoc $< -o $@ $(PANDOCFLAGS)

output:
	mkdir ./output

clean: phony
	rm -rf ./output

open: phony output/book.pdf
	open output/book.pdf
