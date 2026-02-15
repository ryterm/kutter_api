# Contributing to Kutter API

## If you want to contribute to this project, first you need to follow the instructions below.

---

### Code formatting

If you changed some file on the hot path, in any case, you NEED format the code.

To do this, enter on the root path of the repository and run this command:

```sh
cargo fmt
```

That command will format all the code.

---

### Project structure

Before you contribute, make sure that your added code follow that structure:

<details>
<summary>src/app/</summary>
<ul>
<li>Routes</li>
<li>HTTP handler functions</li>
<li>WebSocket handler functions</li>
<li>etc</li>
</ul>
</details>

<details>
<summary>src/db/</summary>
<ul>
<li>Tables management</li>
<li>Queries</li>
<li>etc</li>
</ul>
</details>

<details>
<summary>src/models/</summary>
<ul>
<li>Structs</li>
<li>WebSocket actions</li>
<li>etc</li>
</ul>
</details>

<details>
<summary>src/utils/</summary>
<ul>
<li>Utilitarie functions</li>
<li>Middlewares</li>
<li>Helpers</li>
<li>etc</li>
</ul>
</details>

<details>
<summary>src/</summary>
<ul>
<li>Server stuffs</li>
</ul>
</details>

Any other observation will be warned on Pull Request review

*NOTE: More paths can be added later*

---

### Commits pattern

To commits being approved, follow these guidelines:

#### Commits by context

If you modified more than one file, do not commit all files at once(`git add .`) or one commit per file(`git add file1.ext` ...)

Instead, select and commit files by context. Example:

```sh
git add src/app/file1.rs src/app/file2.rs

git commit -s -m "feat: same context feature for both files"
```

*NOTE: if you need to modify a bunch of files that can't be selected one-by-one, make sure that these modifications has the same context*

#### Signed Off Flag

When you commit, make sure that you are using the Signed Off By flag(-s)

So instead of doing a common commit like this:

```sh
git commit -m "feat: this was added"
```

Do this:

```sh
git commit -s -m "feat: this was added"
```

### Extra

I didn't think about all cases, so, any other case later added can be a result of a suggestion or a bad Pull Request.
