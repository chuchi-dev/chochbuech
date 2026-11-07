# What is mocked

Logging in, writing recipes and the profile are real. Members are zipp
users of the role `member` (no studio), an admin creates them in the
studio, there is no sign up yet; their name, bio and photo are their
page as a cook, see Cooks in `README.md`. The website's server forwards
`/auth/*` to zipp's `/site-auth/*` and `/members/*` to the plugin
zipp-members and keeps the session token in the httpOnly cookie
`chochbuech-session`, see Logins and Recipes of members in `README.md`.
Everything else a visitor does happens in their browser, signed in or
not:

| What | Where | Notes |
|---|---|---|
| Favorites | cookie `chochbuech-favorites` | zipp recipe ids |
| Ratings | cookie `chochbuech-ratings` | hidden: `RATINGS` in `svelte/src/lib/features.ts` turns on the star on a recipe and the stars shown (the `rating` field editors set in zipp) |
| Weekly planner | cookie `chochbuech-planner` | a zipp recipe per day, the shopping list sums their ingredients |

All of it is in `svelte/src/lib/mock.svelte.ts` (`getMock()`), so the
real implementation can replace one class. It is kept by the browser, not
the member: another browser has other favorites, so the pages say "Saved
in this browser." and a guest has them too (`/account/favorites` and
`/account/planner` without the account navigation). Without sign up,
Submit Recipe leads a guest to `/join` (an invitation to write us), the
editor and `/sign-in?register` show the same. Changing the email has no
route of zipp yet, the account page only shows it.

Questions for the real version: a review step for members' recipes
(now a save is live at once), ratings and favorites as their own tables,
sign up.
