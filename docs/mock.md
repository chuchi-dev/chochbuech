# What is mocked

Logging in is real: members are zipp users of the role `member` (no
studio), an admin creates them in the studio, there is no sign up yet. The
website's server forwards `/auth/*` to zipp's `/site-auth/*` and keeps the
session token in the httpOnly cookie `chochbuech-session`, see Logins in
`README.md`. Everything else a member does happens in their browser:

| What | Where | Notes |
|---|---|---|
| Favorites | cookie `chochbuech-favorites` | zipp recipe ids |
| Ratings | cookie `chochbuech-ratings` | the star on a recipe; the stars shown are the `rating` field editors set in zipp |
| Weekly planner | cookie `chochbuech-planner` | a recipe per day, the shopping list sums their ingredients |
| Own recipes | localStorage `chochbuech-recipes` | written in the editor, shown at `/my-recipes/<id>` and in My Recipes; editing a zipp recipe saves a copy (`basedOn`) |
| Deleting a recipe | localStorage `chochbuech-deleted` | hides a zipp recipe from My Recipes |

All of it is in `svelte/src/lib/mock.svelte.ts` (`getMock()`), so the
real implementation can replace one class. It is kept by the browser, not
the member: another browser has other favorites. Cookies are used where the
server should render the state (favorites), localStorage where it is too
big for a cookie. Changing the name or email has no route of zipp yet, the
account page only shows them.

Questions for the real version: recipes written by members as zipp entries
with them as authors and a review step, ratings and favorites as their own
tables, images of members, sign up.
