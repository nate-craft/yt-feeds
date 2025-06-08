# Contributing

YT-Feeds is a project I created for my personal needs, but I am more than open to adding features and modifying the existing code base under two conditions:

1. New additions do not significantly impair performance or introduce excess resource usage
2. New additions do not break the project's existing vision of simplicity and removal of distrations (e.g., shorts, comments, etc)

## Code Guidelines

The entire project uses the standard [rustfmt](https://rust-lang.github.io/rustfmt/) formatting.

Outside of the formatter, try to opt for simple control flow and organization patterns like structs and enums while avoiding 
complex generics and traits when possible. 

## Submitting Changes

1. Fork & clone the repository
2. Checkout a new branch with a descriptive name (e.g., feature_x_username)
3. Commit changes (if using larger commits, write a longer description)
4. Push to your remote branch
5. Submit pull request via GitHub with a description following project guidelines

## Pull Requests Guidelines

Pull requests should feature two things:

1. What has been changed
2. Why it has been added
