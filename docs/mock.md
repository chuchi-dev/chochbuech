# What is mocked

zipp has only the studio's users. Until visitors get accounts (to be
designed), everything a visitor does happens in their browser:

| What | Where | Notes |
|---|---|---|
| Signing in and up | cookie `chochbuech-user` | any email and password work; signing in is "Sören Meier", the cook of two zipp recipes |
| Favorites | cookie `chochbuech-favorites` | zipp recipe ids |
| Ratings | cookie `chochbuech-ratings` | the star on a recipe; the stars shown are the `rating` field editors set in zipp |
| Weekly planner | cookie `chochbuech-planner` | a recipe per day, the shopping list sums their ingredients |
| Own recipes | localStorage `chochbuech-recipes` | written in the editor, shown at `/my-recipes/<id>`; editing a zipp recipe saves a copy (`basedOn`) |
| Deleting a recipe | localStorage `chochbuech-deleted` | hides a zipp recipe from My Recipes |

All of it is in `svelte/src/lib/mock.svelte.ts` (`getMock()`), so the
real implementation can replace one class. Cookies are used where the
server should render the state (header name, favorites), localStorage where
it is too big for a cookie.

Questions for the real version: visitor accounts in zipp (or a service
next to it), recipes written by visitors as zipp entries with a review
step, ratings and favorites as their own tables, images of visitors.
