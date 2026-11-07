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

Members log in on the website with their zipp account (role `member`,
no studio) and write their own recipes there; favorites, ratings and the
weekly planner are a mock in the browser for now, see
[docs/mock.md](docs/mock.md).

### Logins

The browser posts to `/auth/login`, `/auth/logout`, `/auth/reset`
(mails a link to `/reset-password?token=…`) and `/auth/password` (sets the
password of a reset or invite link) and reads `GET /auth/me`. These are
routes of the crelte server (`svelte/src/authRoutes.js`): they only take
JSON from the site's own origin, forward to the cms's `/site-auth/*` with
the visitor's address in `X-Forwarded-For` and keep the session token in
the httpOnly cookie `chochbuech-session`; the browser only gets the user.
The server render reads the user with the cookie (`loadSession` in
`svelte/src/lib/session.svelte.ts`). So the cms runs with
`--trusted-proxy` (the crelte server's address, else all visitors share
one limit of wrong passwords), `--site-url` (the links of the mails) and
`--smtp-url` (without it there are no reset mails).

### Recipes of members

A member's recipes are zipp entries with them as author, written in the
editor (`/recipes/new`, `/recipes/edit/<id>`) and live at once, no
review yet. The browser calls `/members/*` of the crelte server
(`svelte/src/memberRoutes.js`, JSON from the site's own origin, the
image upload as the only multipart request, up to 8 MB), which forwards
to zipp's plugin zipp-members with the session's token: their recipes
(`GET /members/entries?section=recipes`, My Recipes), one of them, a new
one, a change, a delete and `POST /members/assets` for the photo. A
user changes only the recipes they wrote, admins (who can log in on the
website too) as well: on someone else's recipe everybody gets Make it
your own. zipp would let admins edit all of them, the website doesn't.

A member's cook (their public profile, the "by" of their recipes) is the
cooks entry they are the author of (`cookOf.graphql`); the editor sets it
on their recipes (a user without one writes as Anonymous). "Make it
your own" on someone else's recipe (`/recipes/copy/<id>`) starts a new
recipe from it, with its photo. An admin creates the member and their
cook in the studio and sets the cook's author there.

### Local development

```bash
cp craft/.env.example craft/.env
cd cms && cargo run -- serve --dev --site-url en=http://localhost:8080 \
	--public-url http://127.0.0.1:8090
cargo run -- user you@example.com     # a studio login, asks for the password
cargo run -- user lea@example.com --role member   # a website login
cd svelte && npm install && npm run dev
```

- Website `http://localhost:8080`, studio `http://127.0.0.1:8090/admin`,
  GraphiQL `http://127.0.0.1:8090/api/graphql` (only with `--dev`).
- Checks: `cd cms && cargo test && cargo run -- check --queries ../svelte/src/queries`,
  `cd svelte && npm run check`.

### License

MIT or Apache-2.0, see `LICENSE-MIT` and `LICENSE-APACHE`.
