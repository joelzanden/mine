# Mine

`mine` is a lightweight CLI app that adjusts window colors for easy project distinction, randomly selecting a color from the CSS named colors. The app aims to improve your workspace organization and enhance your coding experience by visually differentiating projects with unique colors.

## Features

- Automatically locates project root and updates the settings file
- Randomly selects a color from the CSS named colors
- Enhances visual distinction between projects
- Currently supports Visual Studio Code, with more editors planned for future updates

## Installation

### Precompiled Binaries

You can download precompiled binaries for your operating system from the [releases page](https://github.com/joelzanden/mine/releases).

### Build from Source

If you have Rust and Cargo installed, you can build Mine from source using the following command:

```bash
cargo build --release
```

## Usage

After installing Mine and made sure it's in your system's PATH, you can use it by running the following command in your project directory:

```bash
mine
```

This command will locate the project root, update the settings file with a randomly selected CSS named color, and apply the color to the top portion of the editor window.

## Supported Project Files and Folders

This utility can automatically detect and locate the root of your programming project hierarchy by searching for specific project files and folders. The table below lists the files and folders that are currently supported, categorized by the programming language or framework.

| Language / Framework |                                                                         File(s)                                                                         |
| :------------------: | :-----------------------------------------------------------------------------------------------------------------------------------------------------: |
|       General        | .dockerignore, .editorconfig, .git, .github, .gitignore, .gitlab-ci.yml, .travis.yml, Dockerfile, Jenkinsfile, LICENSE, README.md, Vagrantfile, .vscode |
|       Angular        |                                                                      angular.json                                                                       |
|       ASP.NET        |                                                                      project.json                                                                       |
|          C#          |                                                          .csproj, .fsproj, .sln, project.json                                                           |
|         C++          |                                                                CMakeLists.txt, Makefile                                                                 |
|       Clojure        |                                                                       project.clj                                                                       |
|       Crystal        |                                                                        shard.yml                                                                        |
|         Dart         |                                                                      pubspec.yaml                                                                       |
|        Elixir        |                                                                         mix.exs                                                                         |
|         Elm          |                                                                        elm.json                                                                         |
|        Ember         |                                                                   ember-cli-build.js                                                                    |
|        Erlang        |                                                                      rebar.config                                                                       |
|          F#          |                                                                   paket.dependencies                                                                    |
|          Go          |                                                                         go.mod                                                                          |
|        Groovy        |                                                                      build.gradle                                                                       |
|       Haskell        |                                                                       stack.yaml                                                                        |
|         Java         |                                                                  build.gradle, pom.xml                                                                  |
|        Julia         |                                                                      Project.toml                                                                       |
|         Lua          |                                                                           lua                                                                           |
|        Meteor        |                                                                         meteor                                                                          |
|         Nim          |                                                                         nim.cfg                                                                         |
|         Node         |                                                           lerna.json, package.json, yarn.lock                                                           |
|        OCaml         |                                                                          dune                                                                           |
|         Perl         |                                                                        cpanfile                                                                         |
|         PHP          |                                                                      composer.json                                                                      |
|      PowerShell      |                                                                      psakefile.ps1                                                                      |
|        Python        |                                                   Pipfile, pyproject.toml, requirements.txt, setup.py                                                   |
|          R           |                                                                       DESCRIPTION                                                                       |
|         Ruby         |                                                               .gemspec, Gemfile, Rakefile                                                               |
|         Rust         |                                                                       Cargo.toml                                                                        |
|        Scala         |                                                                        build.sbt                                                                        |
|        Svelte        |                                                                    svelte.config.js                                                                     |
|        Swift         |                                                                      Package.swift                                                                      |
|      TypeScript      |                                                                      tsconfig.json                                                                      |
|         Vue          |                                                                      vue.config.js                                                                      |

## Contributing

If you're interested in contributing to Mine, please feel free to open an issue or submit a pull request on the GitHub repository.
