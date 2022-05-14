## Summary

grok is a (**g**)it b(**rok**)er that responds to events (via webhooks) and _groks_  (tests) the merge-ability of submitted code (Pull Requests) between a (source) repository on one side that anyone may submit to, and a closed (target) repository on the other, where access is limited and requires authentication (SSH keys).

## Status

Status: **proof-of-concept** - validate the feasibility of, and the issues involved with, using a credential set to take privileged actions invoked by non-privileged actors, without comprising those credentials.

## Getting Started

### Installation

To try out the PoC version:

```
$ git clone git@github.com:bsdlabs/grok
$ cargo install --path grok
$ mkdir /usr/local/etc/grok/
$ cp grok/grok.conf /usr/local/etc/grok/
```

Modify the configuration file, run `grok` with `nohup` in the background, and add webhooks in your GitHub repositories for PR events to the grok instance at the _/event_ directory, e.g. http://<grok_address>/event.

### Configuration

Multiple source and target repositories are supported (they just need to come in order since they're zipped):

```
[general]
bind = "127.0.0.1"
port = 5195

[provider]
default = "github"

[repositories]
source = [{repo = "a/source", key = "/path/to/ssh-key"}, {repo = "b/source", key = "/path/to/ssh-key"}]
target = [{repo = "a/target", key = "/path/to/ssh-key"}, {repo = "b/target", key = "/path/to/ssh-key"}]
```

The _provider_ section is not used at the moment.


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
