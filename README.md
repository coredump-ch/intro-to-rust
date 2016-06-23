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
