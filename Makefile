LATEXMK=latexmk
LATEXMK_ARGS=-pdf -e '$$pdflatex=q/xelatex --shell-escape %O %S/'


slides.pdf: slides.tex sections/*.tex
	${LATEXMK} ${LATEXMK_ARGS} slides.tex

clean:
	latexmk -c
	rm -rf _minted-slides/ *.sty *.snm *.vrb *.nav

PHONY: clean
