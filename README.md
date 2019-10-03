# Intro To Rust

Presentation using LaTeX Beamer.

Requirements:

- XeLaTeX
- latexmk
- Fira fonts
- pygmentize
- metropolis theme (part of TeXlive 2016+)

To install the dependencies under Ubuntu 18.04:
```
sudo apt install texlive-xetex texlive-fonts-extra texlive-latex-extra python-pygments
```

## Building

First, install Fira Sans and Fira Mono fonts on your system:

- https://www.fontsquirrel.com/fonts/fira-sans
- https://www.fontsquirrel.com/fonts/fira-mono

Then build the slides:

    git clone https://github.com/coredump-ch/intro-to-rust
    cd intro-to-rust
    make

## [License](LICENSE)

Intro To Rust © by Danilo Bargen and Raphael Nestler

Intro To Rust is licensed under a
Creative Commons Attribution 4.0 International License.

You should have received a copy of the license along with this
work. If not, see http://creativecommons.org/licenses/by/4.0/.
