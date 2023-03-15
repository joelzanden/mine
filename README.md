# Mine

`mine` is a lightweight CLI app that adjusts window colors for easy project distinction, randomly selecting a color from the CSS named colors. The app aims to improve your workspace organization and enhance your coding experience by visually differentiating projects with unique colors.

## Features

- Automatically locates project root and updates the settings file
- Randomly selects a color from the CSS named colors
- Enhances visual distinction between projects
- Currently supports Visual Studio Code, with more editors planned for future updates

## Installation

### Cargo

If you have Rust and Cargo installed, you can install Mine using the following command:

```bash
cargo install mine
```

### Homebrew (macOS)

To install Mine using Homebrew, run the following commands:

```bash
brew tap yourusername/your_repo
brew install mine
```

### Scoop (Windows)

To install Mine using Scoop, run the following command:

```bash
scoop bucket add your_bucket_name https://github.com/yourusername/your_bucket_repo
scoop install mine
```

### Chocolatey (Windows)

To install Mine using Chocolatey, run the following command:

```bash
choco install mine
```

### Debian/Ubuntu (Linux)
To install Mine on Debian/Ubuntu, you can use the apt package manager. First, add the repository and update the package list:


```bash
sudo add-apt-repository ppa:yourusername/your_repo
sudo apt update
```

Next, install Mine with the following command:

```bash
sudo apt install mine
```

## Usage

After installing Mine, you can use it by running the following command in your project directory:

```bash
mine
```

This command will locate the project root, update the settings file with a randomly selected CSS named color, and apply the color to the top portion of the editor window.

## Contributing

If you're interested in contributing to Mine, please feel free to open an issue or submit a pull request on the GitHub repository.

