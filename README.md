## Chochbuech

Chochbuech ("Chuchi Recipes") is a Swiss recipe community: discover recipes,
scale them to your table, save your favorites, plan your week and share your
own creations. Live at [chochbuech.com](https://chochbuech.com).

### How it is built

- `cms/`: the backend, [zipp](https://github.com/zipp-cms/zipp-next), a
  headless CMS whose content model is Rust code (`cms/src/content.rs`).
  Recipes, categories, cooks and the pages are edited in its studio.
- `svelte/`: the website, a [crelte](https://github.com/crelte/crelte)
  (Svelte 5, SSR) project that reads zipp over GraphQL.
- `craft/.env`: crelte reads its settings from there (there is no Craft).

What visitors do themselves (signing in, favorites, ratings, the weekly
planner and writing recipes) is a mock in the browser for now, see
[docs/mock.md](docs/mock.md): zipp only has the studio's users yet.

### Local development

```bash
cp craft/.env.example craft/.env
cd cms && cargo run -- serve --dev --site-url en=http://localhost:8080 \
	--public-url http://127.0.0.1:8090
cargo run -- user you@example.com     # a studio login, asks for the password
cd svelte && npm install && npm run dev
```

- Website `http://localhost:8080`, studio `http://127.0.0.1:8090/admin`,
  GraphiQL `http://127.0.0.1:8090/api/graphql` (only with `--dev`).
- Checks: `cd cms && cargo test && cargo run -- check --queries ../svelte/src/queries`,
  `cd svelte && npm run check`.

### License

MIT or Apache-2.0, see `LICENSE-MIT` and `LICENSE-APACHE`.
