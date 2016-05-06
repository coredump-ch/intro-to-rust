LATEXMK=latexmk
LATEXMK_ARGS=-pdf -e '$$pdflatex=q/xelatex %O %S/'


slides.pdf: slides.tex theme
	${LATEXMK} ${LATEXMK_ARGS} slides.tex

theme: mtheme
	cd mtheme && make sty
	cp mtheme/*.sty .

clean:
	rm slides.pdf

PHONY: theme
