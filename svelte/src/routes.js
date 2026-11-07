// The pages of the frontend that are no entries of zipp: signing in, the
// account and the editor. Logging in and the recipes are zipp's, favorites,
// ratings and the planner mocks (see lib/mock.svelte.ts); their template
// is `account-<type>.svelte`.

/** @param {string} typeHandle @param {string} title */
const page = (typeHandle, title) => () => ({
	sectionHandle: 'account',
	typeHandle,
	title,
});

/** @param {import('crelte/static').StaticRouter} router */
export function staticRoutes(router) {
	router
		.add('/sign-in', page('signIn', 'Sign In'))
		// the links of reset and invite mails: ?token=
		.add('/reset-password', page('resetPassword', 'Your Password'))
		.add('/account', page('myRecipes', 'My Recipes'))
		.add('/account/favorites', page('favorites', 'Favorites'))
		.add('/account/planner', page('planner', 'Weekly Planner'))
		.add('/account/settings', page('settings', 'Account'))
		.add('/recipes/new', page('editor', 'New Recipe'))
		.add('/recipes/edit/:id', csr => ({
			sectionHandle: 'account',
			typeHandle: 'editor',
			title: 'Edit Recipe',
			recipeId: csr.getParam('id'),
		}))
		// Make it your own: a new recipe from someone else's
		.add('/recipes/copy/:id', csr => ({
			sectionHandle: 'account',
			typeHandle: 'editor',
			title: 'New Recipe',
			copyOf: csr.getParam('id'),
		}));
}

/** The links of the account navigation. */
export const ACCOUNT_NAV = [
	['/account', 'My Recipes'],
	['/account/favorites', 'Favorites'],
	['/account/planner', 'Weekly Planner'],
	['/account/settings', 'Account'],
];
