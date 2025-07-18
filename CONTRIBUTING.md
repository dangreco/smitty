# Contribution guidelines

First off, thank you for considering contributing to smitty.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/dangreco/smitty/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Try to do one pull request per change.

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/dangreco/smitty/blob/main/CHANGELOG.md)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!

## Developing

### Set up

This is no different than other Rust projects.

```shell
git clone https://github.com/dangreco/smitty
cd smitty
cargo test
```

#### Nix

To use the development environment configured with Nix, ensure that you have
[Nix](https://nixos.org/download.html) installed and run:

```shell
nix develop
```

Optionally, you can use [direnv](https://direnv.net/) to automatically activate the development environment when you enter the project directory:

```shell
direnv allow
```

### Useful Commands

- Build:

  ```shell
  just build
  ```

- Run Clippy:

  ```shell
  just clippy
  ```

- Run all tests:

  ```shell
  just test
  ```

- Check to see if there are code formatting issues

  ```shell
  just fmt-check
  ```

- Format the code in the project

  ```shell
  just fmt
  ```
