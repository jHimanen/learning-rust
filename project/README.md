# project/

Your capstone lives here: **minipolars**, a small dataframe engine written from scratch.

You create it yourself in chapter 2 with:

```sh
cargo new project/minipolars
```

Everything inside `project/minipolars/` is yours: `main` never touches it. Chapters with a
★ milestone ship acceptance tests in `chapters/chNN-*/milestone/`. You copy those into
`project/minipolars/tests/` when the chapter tells you to.

(Any directory you put in `project/` that contains a `Cargo.toml` becomes a member of the
workspace automatically, via the `project/*` glob in the root `Cargo.toml`.)
