# find-semantic-release-config

[![Build Status]](https://github.com/EricCrosson/find-semantic-release-config/actions/workflows/release.yml)

[build status]: https://github.com/EricCrosson/find-semantic-release-config/actions/workflows/release.yml/badge.svg?event=push

**find-semantic-release-config** is a library to locate a project's semantic-release configuration.

The configuration rules, according to the [semantic-release readme]:

semantic-release’s options, mode and plugins can be set via either:

- A `.releaserc` file, written in YAML or JSON, with optional extensions: `.yaml`/`.yml`/`.json`/`.js`/`.cjs`
- A `release.config.(js|cjs)` file that exports an object
- A `release` key in the project's package.json file

[semantic-release readme]: https://github.com/semantic-release/semantic-release/blob/master/docs/usage/configuration.md#configuration-file
