.PHONY: phony

BOOK_NAME="Leetcode in Rust"
GRAPHVIZ_FILES = $(shell find ./graphviz -name '*.dot')
FIGURES = $(shell find ./figures -name '*.svg')
CHAPTERS = $(shell find ./chapters -name '*.md')

PANDOCFLAGS =                        \
	--metadata title=$(BOOK_NAME)      \
  --table-of-contents                \
  --from=markdown                    \
  --number-sections                  \
	--top-level-division=chapter       \
	--filter=pandoc-include            \
	--filter=pandoc-include-code       \
  --indented-code-classes=javascript

HTML_FLAGS =                         \
	--template=./templates/book.html   \
	--self-contained

PDF_FLAGS =                          \
  --pdf-engine=xelatex               \
	--template=eisvogel                \
	-V toc-own-page=false              \
  -V mainfont="Palatino"             \
  -V documentclass=krantz            \
  -V papersize=A4                    \
	-V book=true                       \
	-V titlepage=true                  \
	-V classoption=oneside

html: phony output/book.html

epub: phony output/book.epub

pdf: phony output/book.pdf

docx: phony output/book.docx

figures:
	./bin/figures

output/%.pdf: %.md $(FIGURES) $(CHAPTERS) Makefile | output figures
	pandoc $< -o $@ $(PDF_FLAGS) $(PANDOCFLAGS)

output/%.epub: %.md $(FIGURES) $(CHAPTERS) Makefile | output figures
	pandoc $< -o $@ $(PANDOCFLAGS)

output/%.html: %.md $(FIGURES) $(CHAPTERS) Makefile templates/book.html | output figures
	pandoc $< -o $@ $(HTML_FLAGS) $(PANDOCFLAGS)

output/%.docx: %.md $(FIGURES) $(CHAPTERS) Makefile | output figures
	pandoc $< -o $@ $(PANDOCFLAGS)

output:
	mkdir ./output

clean: phony
	rm -rf ./output

open: phony output/book.pdf
	open output/book.pdf
