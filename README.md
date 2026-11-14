## Chochbuech

Chochbuech ("Chuchi Recipes") is a Swiss recipe community: discover recipes,
scale them to your table, save your favorites, plan your week and share your
own creations. Live at [chochbuech.com](https://chochbuech.com).

### How it is built

- `cms/`: the backend, [zipp](https://github.com/zipp-cms/zipp-next), a
  headless CMS whose content model is Rust code (`cms/src/content.rs`).
  Recipes, categories, the pages and the users (the cooks) are edited in
  its studio.
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
password of a reset or invite link), reads `GET /auth/me` and changes the
user's profile with `PUT /auth/me` (`{ name, fields: { bio, photo } }`,
the account page). These are
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

"Make it your own" on someone else's recipe (`/recipes/copy/<id>`)
starts a new recipe from it, with its photo. An admin creates the member
in the studio.

### Cooks

The cook of a recipe is always its author, like WordPress' author pages:
zipp's users have a profile (the user fields `bio`, `photo` and
`memberSince` in `cms/src/content.rs`, their name and a slug) and a page
`/cooks/<slug>` while a recipe of theirs is live, so admins and new
members are on no page. GraphQL gives the recipe's `author` (`url` only
while they are public; the cards and the recipe show Anonymous without
one) and `user(slug:)` (`cook.graphql`), their recipes are
`recipes.graphql` with `$author`. The page is a route of crelte's static
router (`svelte/src/routes.js`, template `cooks-cook.svelte`), a 404 for
a user without a page. Members change their name, bio and photo on the
account page; the slug and `memberSince` are an admin's, in the studio's
user page. A recipe gets another author in the studio (admins). The
users are only in GraphQL for `serve`'s token (`ZIPP_GRAPHQL_TOKEN`) or a
token of the studio whose GraphQL schema has `.users()` (the schema
`website` in `cms/src/content.rs`).

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

### Deploy

The site runs on a server with the hosting panel genco, which deploys the
branch `main` with `genco.toml`: the cms on `admin.chochbuech.com`, the
website on `chochbuech.com`. The setup, the settings (`cms/.env.example`,
`craft/.env.example`), the first data and the backups are in
[docs/deploy.md](docs/deploy.md).

### License

MIT or Apache-2.0, see `LICENSE-MIT` and `LICENSE-APACHE`.
