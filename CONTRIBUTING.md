# Contributing
This document is heavily WIP.

If you wanna contribute to `eza`, here are the absolute basics:
- your commit summary should follow conventional commits.
- your commits should be separated into small, logical chunks.
- reviewers may ask you to rebase your commits into more sensible chunks.
- your PR will need to pass CI and local `cargo test`.
- you may be asked to refactor parts of your code by reviewers.

## Commit Messages
A common commit message contains at least a summary and reference with
closing action to the corresponding issue if any, and may also include a
description and signature.

### Summary
For you commit messages, please use the first line for a brief summary what
the commit changes. Try to stay within the 72 char limit and prepend what type
of change. See the following list for some guidance:
- feat: adds a new feature to eza
- feat(zsh): adds something to zsh completion
- refactor: revises parts of the code
- doc(readme): revise the README
- doc(man): revision of the man pages
- fix: bugfix in the code base
- fix(ci): bugfix in the continuos integration
- ...

Note that this list is not complete and there may be cases where a commit
could be characterized by different types, so just try to make your best
guess. This spares the maintainers a lot of work when merging your PR.

### Description
If you commit warrants it due to complexity or external information required
to follow it, you should add a more detailed description of the changes,
reasoning and also link external documentation if necessary. This description
should go two lines below the summary and except for links stay in the 80 char
limit.

### Issue Reference
If the commit resolves an issue add: `Resolves #abc` where `abc` is the issue
number. In case of a bugfix you can also use `Fixes #abc`.

### Signature
You may add a signature at the end two lines below the description or
issue reference.

### Example
Here is an example of a commit message that follows these rules (mostly):
```
fix: TextCell building of detailed grid view for  hyperlink and icon options

The hyperlink option adds an escape sequence which in the normal TextCell
creation also becomes part of the length calculation. This patch applies
the same logic the normal grid already did, by using the filenames bare
width when a hyperlink is embedded. It also respects the ShowIcons
option just like the normal grid view.

Resolves #129
```
