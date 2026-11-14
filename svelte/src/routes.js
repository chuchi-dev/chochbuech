// The pages of the frontend that are no entries of zipp: signing in, the
// account and the editor. Logging in and the recipes are zipp's, favorites,
// ratings and the planner mocks (see lib/mock.svelte.ts); their template
// is `account-<type>.svelte`. And the cooks: zipp's users with a live
// recipe, `cooks-cook.svelte`.

import cookQuery from './queries/cook.graphql';

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
		// no sign up yet: where Submit Recipe leads a guest
		.add('/join', page('join', 'Share your Recipes'))
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
		// a private recipe (unpublished, only its author sees it)
		.add('/my-recipes/:id', csr => ({
			sectionHandle: 'account',
			typeHandle: 'myRecipe',
			title: 'Private Recipe',
			recipeId: csr.getParam('id'),
		}))
		// Make it your own: a new recipe from someone else's
		.add('/recipes/copy/:id', csr => ({
			sectionHandle: 'account',
			typeHandle: 'editor',
			title: 'New Recipe',
			copyOf: csr.getParam('id'),
		}))
		// the public page of a cook, a user with a live recipe: their
		// uri in zipp (user pages, the `url` of a recipe's author)
		.add('/cooks/:slug', async csr => {
			const slug = decodeURIComponent(csr.getParam('slug') ?? '');
			const data = /** @type {any} */ (
				await csr.query(cookQuery, { slug })
			);
			if (!data.cook)
				return { sectionHandle: 'error', typeHandle: '404' };
			return {
				sectionHandle: 'cooks',
				typeHandle: 'cook',
				title: data.cook.name,
				cook: data.cook,
			};
		});
}

/** The links of the account navigation. */
export const ACCOUNT_NAV = [
	['/account', 'My Recipes'],
	['/account/favorites', 'Favorites'],
	['/account/planner', 'Weekly Planner'],
	['/account/settings', 'Account'],
];

/** The pages a guest has too, kept by the browser: their navigation. */
export const GUEST_NAV = [
	['/account/favorites', 'Favorites'],
	['/account/planner', 'Weekly Planner'],
];
