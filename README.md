# Resume builder

## Install

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# LaTeX
## Arch linux
sudo pacman -Sy texlive-core
## Mac - also may need to add pdflatex to path (located in /Library/TeX/texbin)
brew install --cask basictex
## Ubuntu
sudo apt -y install texlive
# Get the code
git clone --depth 1 https://github.com/spencewenski/resume_generator.git
```

## Running
```bash
cargo run -- cargo run -- -i $HOME/Desktop/resume/resume.toml -o resume -d $HOME/Desktop/resume/output
```

## Design
- write resume as json (or toml?)
- provide resume data as a list of items to render
- each item specifies the 'type' of its data
    - this defines what data it can provide to the renderer
    - e.g., an 'Education' item can provide the school, major, start/end dates, etc.
- each type knows how to render itself
    - ideally we could define the renderers in the json, but that's way more complicated than I want to deal with right now (would require inventing markup language, which is overkill for me right now)
- each type can render itself in various formats, e.g. latex/pdf, text, html, markdown
- possibly will create a github action to deploy the result resume as a page on my website
