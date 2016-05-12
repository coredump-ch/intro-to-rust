LATEXMK=latexmk
LATEXMK_ARGS=-pdf -e '$$pdflatex=q/xelatex --shell-escape %O %S/'


slides.pdf: slides.tex theme
	${LATEXMK} ${LATEXMK_ARGS} slides.tex

theme: mtheme
	cd mtheme && make sty
	cp mtheme/*.sty .

clean:
	latexmk -c
	rm -rf _minted-slides/ *.sty *.snm *.vrb *.nav

PHONY: theme clean
