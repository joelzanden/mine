use std::path::Path;
use std::path::PathBuf;

pub(crate) fn path_is_separator(path: &Path) -> bool {
    path.parent()
        .map_or(false, |parent| path.as_os_str() == parent.as_os_str())
}

pub fn get_project_dir_path() -> Result<PathBuf, &'static str> {
    let mut path = std::env::current_dir().unwrap();
    let home = dirs::home_dir().unwrap();

    while !path_is_separator(&path) && path != home {
        if we_are_in_a_project_dir(&path) {
            return Ok(path);
        }
        path.pop();
    }

    Err("Could not find a project directory. Are you in a project directory?")
}

pub fn we_are_in_a_project_dir(path: &Path) -> bool {
    let project_files = [
        // General
        ".vscode",
        ".git",
        ".github",
        ".gitlab-ci.yml",
        ".editorconfig",
        ".gitignore",
        "README.md",
        "LICENSE",
        "Dockerfile",
        ".dockerignore",
        "Vagrantfile",
        ".travis.yml",
        "Jenkinsfile",
        // Rust
        "Cargo.toml",
        // Python
        "pyproject.toml",
        "requirements.txt",
        "setup.py",
        "Pipfile",
        // Node
        "package.json",
        "yarn.lock",
        "lerna.json",
        // Java
        "pom.xml",
        "build.gradle",
        // C#
        "project.json",
        ".csproj",
        ".fsproj",
        ".sln",
        // C++
        "CMakeLists.txt",
        "Makefile",
        // Go
        "go.mod",
        // PHP
        "composer.json",
        // Ruby
        "Gemfile",
        ".gemspec",
        "Rakefile",
        // Swift
        "Package.swift",
        // Haskell
        "stack.yaml",
        // Lua
        "lua",
        // Perl
        "cpanfile",
        // Scala
        "build.sbt",
        // Elixir
        "mix.exs",
        // Elm
        "elm.json",
        // Erlang
        "rebar.config",
        // Dart
        "pubspec.yaml",
        // Julia
        "Project.toml",
        // Crystal
        "shard.yml",
        // Nim
        "nim.cfg",
        // OCaml
        "dune",
        // R
        "DESCRIPTION",
        // F#
        "paket.dependencies",
        // Clojure
        "project.clj",
        // Groovy
        "build.gradle",
        // PowerShell
        "psakefile.ps1",
        // TypeScript
        "tsconfig.json",
        // Vue
        "vue.config.js",
        // Angular
        "angular.json",
        // Svelte
        "svelte.config.js",
        // Ember
        "ember-cli-build.js",
        // Meteor
        "meteor",
        // ASP.NET
        "project.json",
    ];

    project_files.iter().any(|file| path.join(file).exists())
}
