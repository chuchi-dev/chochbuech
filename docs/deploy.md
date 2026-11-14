# Deploying chochbuech

chochbuech.com runs on gs2 (s2.goodserver.ch), a server with the hosting
panel [genco](https://github.com/chuchi-dev/genco): genco deploys the
branch `main` with the steps of `genco.toml` and runs the cms and the
website as two apps of the group `chochbuech`. Changes on the server go
through genco's agent (noah proposes, the operator approves); textrem
runs the same way.

## What runs

| Domain | Kind | Root | Command |
| --- | --- | --- | --- |
| `admin.chochbuech.com` | rust | `current/cms` | `set -a; . ./.env; set +a; ZIPP_LISTEN="$HOST:$PORT" exec ./chochbuech-cms serve` |
| `chochbuech.com` | node | `current/svelte` | `node server.cjs`, public folder `current/svelte/dist/public` |

- The cms serves the studio (`/admin`), GraphQL (`/api/graphql`, only
  with the token) and the files. Its settings are `shared/cms/.env` (see
  `cms/.env.example`), the database, the files and its own backups are
  `shared/cms/data`. It applies new migrations when it starts.
- The website is crelte's node server; it reads `shared/craft/.env` (see
  `craft/.env.example`). Its `ENDPOINT_URL` is the cms's port on
  127.0.0.1 (genco gives an app port 20000 + the domain's id, `genco
  tree` shows the id), not the cms's public address: the logins
  (`/auth/*`) and the editor (`/members/*`) go to the same address, and
  the cms only believes the visitor's address in `X-Forwarded-For` from
  127.0.0.1 (`ZIPP_TRUSTED_PROXY`). Through nginx every visitor would be
  the server's own address and share one limit of wrong passwords.
- So the cms runs without "Restart without downtime": its second slot
  would listen on another port (30000 + id) than `ENDPOINT_URL`, and two
  processes on one SQLite database during a migration is no good idea
  either. A deploy restarts it in a few seconds. The website may have
  it.
- `genco.toml`: the cms builds on a genco runner (`cargo build
  --release --locked`, only the binary comes back), `chochbuech-cms
  check --queries` checks the website's queries against the schema,
  then `npm ci` and `npm run build` in `svelte/`. `shared` keeps
  `cms/.env`, `cms/data` and `craft/.env` across releases.

## Setting it up

1. Client Meier Labs, group `chochbuech`, repo
   `git@github.com:chuchi-dev/chochbuech.git`, branch `main`. The group's
   deploy key goes to the GitHub repo (read only).
2. The domains as in the table above (`genco domain add` with `--kind`,
   `--root`, `--command`, `--public`), certificates of Let's Encrypt.
   The DNS of `chochbuech.com`, `www.chochbuech.com` and
   `admin.chochbuech.com` points to the server first; `--www true` on
   `chochbuech.com` redirects www there. No noindex.
3. `shared/cms/.env` from `cms/.env.example` with a new
   `ZIPP_GRAPHQL_TOKEN` (`openssl rand -hex 32`), `shared/craft/.env`:

   ```
   ENDPOINT_URL=http://127.0.0.1:<20000 + id of admin.chochbuech.com>/api/graphql
   FRONTEND_URL=https://chochbuech.com
   CRAFT_BASE_CP_URL=https://admin.chochbuech.com
   ENDPOINT_TOKEN=<the same token>
   ```

   Both mode 600, owned by the group's user.

   The recipe assistant (the editor's Import from a website) needs an
   Anthropic API key: `shared/cms/data/anthropic-key` (mode 600, the
   group's user) and in `shared/cms/.env`
   `CHOCHBUECH_ANTHROPIC_KEY_FILE=data/anthropic-key` (relative to
   `current/cms`, whose `data` is the shared one). `CHOCHBUECH_IMPORT_MODEL`
   picks the model (`claude-haiku-5-5` without it, `claude-sonnet-5-5`
   writes better and costs more). Without a key the import answers
   "The import is not configured"; the rest of the site works. The cms
   fetches the pages members import itself, so it needs outgoing
   http(s).
4. Mail (the password reset and invite mails): a send-only address
   `noreply@chochbuech.com` (`genco mail add-domain chochbuech.com
   --group chochbuech --send-only`, `genco mail send-only add
   noreply@chochbuech.com --env chochbuech/admin.chochbuech.com`), which
   writes `ZIPP_SMTP_URL` into the cms's environment, plus the SPF and
   DKIM records of `genco mail dns chochbuech.com`. Until then
   `ZIPP_SMTP_URL=smtp://127.0.0.1:25` in `shared/cms/.env` with a
   sender the server may use (textrem: `noreply@s2.goodserver.ch`).
   `chochbuech-cms mail <to>` in `current/cms` sends a test mail.
5. `genco deploy chochbuech`, then the data (below) while the cms is
   stopped, then start both apps.

## The first data

The content comes as a package of `chochbuech-cms backup` (the database
and the files of the cms it was made on):

```bash
# where the content is, with the cms of the same version
chochbuech-cms --db zipp.db backup /tmp/chochbuech-import --files files
tar -C /tmp/chochbuech-import -czf chochbuech-import.tar.gz .
sha256sum chochbuech-import.tar.gz
```

On the server, as the group's user in `shared/cms` with the cms stopped,
after checking the sha256 and unpacking outside the web roots:

```bash
../../current/cms/chochbuech-cms --db data/zipp.db restore <dir>/<backup folder> --files data/files
../../current/cms/chochbuech-cms --db data/zipp.db user <email>   # the studio admin, password on stdin
```

The logins of the package keep their passwords: give the test logins of
a demo new random ones (`user <email> --role member`, a password nobody
knows) or leave them out of the package. Delete the archive afterwards.

## Backups

- genco backs up the group every night (restic: files, and
  `shared/cms/data/zipp.db` as a `sqlite3 .backup` copy in
  `.genco-sqlite/`), see genco's OPERATIONS.md "Restoring one site".
- The cms makes its own backup a day into `shared/cms/data/backups`
  (`ZIPP_BACKUP_DIR`, the last 7), the files as hard links to the
  last one. `chochbuech-cms restore <backup> --files data/files` with
  the cms stopped puts one back.

## A new release

Merge into `main`, then `genco deploy chochbuech` (or the Deploy tab). A
change of the schema needs a migration of its own (`cargo run --
makemigration`) once the site is live, see AGENT.md.
