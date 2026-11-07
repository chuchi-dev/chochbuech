// The pages of the frontend that are no entries of zipp: signing in, the
// account and the editor. They are mocks for now (see lib/mock.svelte.ts),
// their template is `account-<type>.svelte`.

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
		// a recipe of the editor, only in this browser
		.add('/my-recipes/:id', csr => ({
			sectionHandle: 'account',
			typeHandle: 'localRecipe',
			title: 'Recipe',
			recipeId: csr.getParam('id'),
		}));
}

/** The links of the account navigation. */
export const ACCOUNT_NAV = [
	['/account', 'My Recipes'],
	['/account/favorites', 'Favorites'],
	['/account/planner', 'Weekly Planner'],
	['/account/settings', 'Account'],
];
