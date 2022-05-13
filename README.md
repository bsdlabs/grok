## Summary

grok is a (**g**)it b(**rok**)er that responds to events (via webhooks) and _groks_  (tests) the merge-ability of submitted code (Pull Requests) between a (source) repository on one side that anyone may submit to, and a closed (target) repository on the other, where access is limited and requires authentication (SSH keys).

## Status

Status: **proof-of-concept** - validate the feasibility of, and the issues involved with, using a credential set to take privileged actions invoked by non-privileged actors, without comprising those credentials.

## Roadmap

- [ ] Add error handling (failed merge, sync failure, etc)
- [ ] Take actions (comment, close, etc) on GitHub objects via API (pull requests)
- [ ] Refactor merge method. Currently implemented `std::process::Command`. Rewrite using `libgit2`
- [ ] Add logging
- [ ] Add tests
- [ ] Create mechanism to execute `pre-push` actions or steps (eg: CI, etc)
- [ ] Document code and provide examples
- [ ] Improve the README to cover:
          - Documentation
          - Installation
          - Configuration
          - Contributing
