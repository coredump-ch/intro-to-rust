# Intro To Rust

Presentation using LaTeX Beamer.

Requirements:

- XeLaTeX
- latexmk
- Fira fonts
- pygmentize

To install the dependencies under Ubuntu 16.04:
```
sudo apt install texlive-xetex texlive-fonts-extra python-pygments
```

## Building

First, install Fira Sans and Fira Mono fonts on your system:

- https://www.fontsquirrel.com/fonts/fira-sans
- https://www.fontsquirrel.com/fonts/fira-mono

Then build the slides:

    git clone --recursive https://github.com/coredump-ch/intro-to-rust
    make

## [License](LICENSE)

Intro To Rust © by Danilo Bargen and Raphael Nestler

Intro To Rust is licensed under a
Creative Commons Attribution 4.0 International License.

You should have received a copy of the license along with this
work. If not, see http://creativecommons.org/licenses/by/4.0/.
