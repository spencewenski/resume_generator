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

## Other helpful usage tips
Create a script or alias to run the tool from any directory. Script version:
```sh
#!/bin/sh

cargo run --manifest-path $HOME/projects/resume_generator/Cargo.toml -- -i $HOME/projects/resume/resume.toml -o resume -d $HOME/projects/resume/output "$@"
```

Regenerate the resume when the config file is updated:
```sh
# Install inotify-tools first
## Arch linux
sudo pacman -Sy inotify-tools
# https://superuser.com/questions/181517/how-to-execute-a-command-whenever-a-file-changes
while inotifywait -e close_write resume.toml; do resume; done
```

## Design
- Resume data is stored in a toml file
  - See `tst/test_resume.toml` for a sample
  - Todo: support other formats, e.g. json, yaml, etc
- Each element of the resume is represented as a struct and deserialized from the config file
- The `Renderer` trait is implemented for each format
  - E.g. `impl Renderer<Resume, String> for TextRenderer` renders the Resume struct to a String
- The `Renderer` trait has type parameters so it can be implemented for each individual element of the resume
  - E.g. `impl Renderer<PersonalInfo, String> for TextRenderer` renders the PersonalInfo element to a String
- There are various types of renderers, and each will render the relevant elements of the Resume.
  - Current renderers: TextRenderer, MarkdownRenderer, PdfRenderer, GitHubRenderer
- Renderers can also be composed. For example, the GitHubRenderer uses the MarkdownRenderer to render the
  elements it cares about

## Extending / using as a rust library
- This project is mainly designed as a 'library' repository, so it should be easy to include it in another rust
  project and add custom Renderer implementations
- Todo: make sure this works and add instructions

## GitHub Workflows
### Build and test
The first workflow that builds the project and runs the test.

### Trigger resume render
This workflow runs after the build and test workflow completes. It invokes a `repository_dispatch` on a separate
repository that contains the resume data (this resume repo can be private). In order to use this workflow in a
fork of this generator package:

- Create a personal access token in the developer settings
- Add this as a secret in the generator fork with name `API_TOKEN`
- Create another repository to contain the resume data
- Add the resume repo as a secret in the generator fork with name `RESUME_REPO`
- Add a workflow to the resume repo that has the `repository_dispatch` trigger and builds the resume. Sample:
```yaml
# Renders the resume, archives the artifacts, and updates the GitHub profile

name: Render Resume

# Controls when the action will run.
on:
  # Triggers the workflow on push or pull request events but only for the main branch
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

  # Allows you to run this workflow manually from the Actions tab
  workflow_dispatch:
  # Allows other workflows to invoke this workflow
  repository_dispatch:

# A workflow run is made up of one or more jobs that can run sequentially or in parallel
jobs:
  # This workflow contains a single job called "build"
  build:
    # The type of runner that the job will run on
    runs-on: ubuntu-latest

    # Steps represent a sequence of tasks that will be executed as part of the job
    steps:
      # Checks-out your repository under $GITHUB_WORKSPACE, so your job can access it
      - name: Checkout resume
        uses: actions/checkout@v2
        with:
          token: ${{ secrets.API_TOKEN }}

      - name: Checkout resume generator
        uses: actions/checkout@v2
        with:
          repository: spencewenski/resume_generator
          path: resume_generator

      - name: Install LaTeX
        run: sudo apt -y install texlive

      - name: Render resume
        working-directory: ./resume_generator
        run: cargo run -- -i ../resume.toml -o resume -d ../output

      - name: Archive rendered resume
        uses: actions/upload-artifact@v2
        with:
          name: resume
          path: output
```

- You can also update another repo (e.g. your GitHub profile repo) with content generated in the above steps:
```yaml
      # ... continued from the above
      # Replace placeholders, e.g. 'username', with your data
      - name: Checkout profile repo
        uses: actions/checkout@v2
        with:
         repository: username/username
         path: username
         token: ${{ secrets.API_TOKEN }}

      - name: Update profile
        working-directory: ./username
        run: |
          cp ../output/resume-github.md ./README.md
          git -c user.name="Your Name" -c user.email="your.email@example.com" commit -am "Update profile" --author="Your Name"
          git push origin main
```
