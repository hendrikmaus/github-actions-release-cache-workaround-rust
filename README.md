# Workaround to Cache Releases Using GitHub Actions and Packages

*Please mind that I created a set of GitHub Aactions to re-use this concept in [hendrikmaus/custom-cache-action](https://github.com/hendrikmaus/custom-cache-action).*

For all the details, please see my blog post: [GitHub Actions: How to Implement Caching For Releases (Rust as Example)](https://blog.hendrikmaus.dev/github-actions-release-caching/).

## Results

Initial release [`1.0.0`](https://github.com/hendrikmaus/github-actions-release-cache-workaround-rust/actions/runs/1484389352) took `1m 28s` to build the release artifact.

A subsequent release [`1.1.0`](https://github.com/hendrikmaus/github-actions-release-cache-workaround-rust/runs/4273117553), which did not include any code change to be fair, took `7.99s` to build the release artifact.
