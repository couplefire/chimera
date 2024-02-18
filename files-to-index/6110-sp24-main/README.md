# 6.1100 Decaf Compiler Rust

In this repo you will find a skeleton for the Decaf compiler project. You may organize this project in any way you see fit as long as the build.sh and run.sh files build and run your compiler respectively.

## Development Setup

### Rust

You will need to install Rust and Cargo. It is recommended to use `rustup`. For more details, see: https://www.rust-lang.org/tools/install.

### Cloning the Repository

We use Git Submodules to clone the `public-tests` folder. Clone this repository, and all submodules, with:

```
git clone --recurse-submodules https://github.com/thecodingwizard/6110-sp24.git
```

If you've already cloned the repository but forgot to use `--recursive-submodules`, run:

```
git submodule update --init --recursive
```

### pre-commit hooks

We use [pre-commit](https://pre-commit.com/) to enforce formatting before committing.

```
pip install pre-commit
pre-commit install
```

## Tests

`test.sh` runs both cargo tests (unit tests, etc.) and integration tests (tests provided by the class).

To delete unused snapshots for cargo tests, run:

```
cargo insta test --unreferenced=delete
```
