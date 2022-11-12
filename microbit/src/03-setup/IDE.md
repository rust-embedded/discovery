# Getting the best of your IDE

All codes in this book works assume that you use a simple terminal to build your code
run it, and interact with it. It also makes no assumption about your text editor.

However, you may have your favourite IDEs, providing you auto-complete, type annotation,
your preferred shortcuts and much more. This section explains how to get the best
of your IDE using the code obtained from this book's repo.

# Auto-completion, type annotation, and more

Some IDEs fail to understand the code, because it fails to determine whether a term
is defined in microbit or microbit-v2 codebase. If you fail to get auto-completion to work,
you may want to try to edit the `Cargo.toml` files you encounter through this book, and remove
all references to the version of microbit you are not using. That is:
* in Cargo.toml you must remove the dependency and feature you do not use
* in code, you must remove the part of the code guarded by `#[cfg(feature = "vI")]`, and the guard

# IDEs configuration

Below, we explain how to configure your IDE to get the best out of this book.
If your IDE is not listed below, please improve this book by adding a section, so that the next
reader get the best experience out of it.

## How to build with IntelliJ

When editing IntelliJ build configuration, here are a few non-default values:
* you should edit the command. When this book tells you to run `cargo embed FLAGS`,
you'll need to replace the default value `run` by the command `embed FLAGS`,
* You should check "Emulate terminal in output console". Otherwise, your program will fail to print text to a terminal
* You should ensure that the working directory is `microbit/src/N-name`, with `N-name` the directory of the chapter you
are reading. You can not run from the `src` directory since it contains no cargo file.