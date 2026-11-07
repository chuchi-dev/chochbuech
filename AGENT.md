# Working on chochbuech

Read `README.md` first. The backend is zipp (`cms/`), the website crelte
(`svelte/`).

## The cms

- `cms/src/content.rs` is the content model. While the site isn't live, a
  change of the schema regenerates `migrations/0001_initial.json`
  (`cargo run -- makemigration`, delete the old file first) instead of adding
  migrations; real migrations start with the launch.
- `cms/src/tests.rs` runs the website's GraphQL queries
  (`include_str!` of `svelte/src/queries`) against a TestServer: when a query
  or the model changes, keep them passing and add a case for new behaviour.
- `cargo run -- check --queries ../svelte/src/queries` checks every query
  against the schema.

## The website

- Svelte 5 runes, crelte 0.6. `svelte/src/App.svelte` owns the header, the
  footer, the toasts, the globals (`global.graphql`, in its
  `loadGlobalData`), the session (`lib/session.svelte.ts`, `getSession()`:
  the logged in zipp user) and the mock (`lib/mock.svelte.ts`, a context:
  favorites, ratings, the planner).
- The session token never reaches the browser: it is an httpOnly cookie of
  the routes in `svelte/src/authRoutes.js` and `memberRoutes.js` (crelte's
  server router), see Logins and Recipes of members in `README.md`. The
  browser's side of the recipes is `lib/members.ts`.
- crelte keeps the answers of queries in the browser for the whole visit:
  after a change call `forgetQueries` (`lib/members.ts`), else the next
  page shows the old content.
- Templates are `svelte/src/templates/<section>-<type>.svelte`. Pages that
  are no entry of zipp (signing in, a new password, the account, the
  editor) are routes of
  crelte's static router in `svelte/src/routes.js` with the section
  `account`; adding a template file needs a restart of the vite dev server.
- Queries are self-contained files in `svelte/src/queries/` (no fragments
  across files); crelte runs them on its node server by name, so the client
  can call them too (`getCrelte().query(...)`).
- Components: `Button` (the pill with its ring), `RoundButton`, `Icon`
  (Lucide paths), `Pills`, `RecipeCard`/`RecipeGrid`, `Stars`,
  `SearchField`, `Richtext`; the recipe page is `components/recipe/`, the
  home blocks `components/home/`, the editor `components/editor/`.
- SCSS: `sass/vars.scss` (breakpoints, mixins; prepended to every component,
  no CSS output), tokens in `sass/main.scss`. Mobile first, never a bare
  `:hover` (always inside `@include hover`), colors only as tokens.
- The design is the Figma file "Chuchi Recipes v2" (desktop 1600 px, mobile
  375 px); its hand writing font Smoothy is replaced by Caveat (OFL).
