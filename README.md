# Gametest
[![Build Status](https://travis-ci.org/macrohardmx/gametest.svg?branch=master)](https://travis-ci.org/macrohardmx/gametest)

This is my experiment into rust game development.

## Running and Compiling

### Windows
To run, all you need to do is [install `rust` and `cargo`](https://www.rust-lang.org/tools/install) and run:
```
$ cargo run
```

### Ubuntu
They're pretty similar to those for Windows, but you need to install these dependencies
first:

```
$ sudo apt install -y libasound2-dev libudev-dev
```

And then, after installing `rust` and `cargo`, run:
```
$ cargo run
```

## Contributing
If you're part of the team contributing to this project, here's some guidelines on how to work on the project

1. **Pick a task**: Grab any task in [the project](https://github.com/orgs/macrohardmx/projects/1) under the To Do column, assign it to yourself, and move it to the "in progress".
2. **Work on it locally**: Locally, create a branch and make commits freely. Name it by following this convention: `$PREFIX/$DESC-$NUM`, where `$PREFIX` is a prefix denoting the kind of issue, `$DESC` is a short description of the task and `$NUM` is the issue number. 
 As an example, if you're working on a feature, dictated by issue #9999 about adding fire bullets to a character, you might name the branch `feat/fire-9999`. `$PREFIX` includes things like:
  - `feat` for feature work
  - `fix` for technical improvements and fixes
  - `bug` for bugs
3. **Sign all your commits**: Please sign all your commits using a PGP key. They are required for you to be able to commit your changes. You can [read this](https://help.github.com/en/github/authenticating-to-github/signing-commits) for instructions on how to set up signed commits on your local git, as well as [these instructions](https://riseup.net/en/security/message-security/openpgp/gpg-keys) on how to create your PGP private key. You can also confirm that all the commits are signed by running `$ git log --show-signature` from your terminal.
4. **Create a pull request**: After you're done making your changes, push your branch with `$ git push -u origin $BRANCH_NAME` where `$BRANCH_NAME` is the name of your new branch (or use whatever GUI you have for git). You will then be able to open this repo online and create a new pull request with that new branch.
5. **Merge the change**: Finally, when merging, remember to use the "Create a merge commit option". This will make sure the branch ends up pointing into master. Also make sure to include a phrase saying that which issue you resolved. As an example, if you're still working on issue #9999 from before, make sure the description of the commit contains the phrase "fixed #9999". This will make sure GitHub closes the issue for you when you merge.

Feel free to reach out to @rdelfin or anyone on the team for questions. Happy coding!
