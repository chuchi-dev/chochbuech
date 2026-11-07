// Everything a visitor does is a mock for now: zipp has only the studio's
// users. The session, favorites and ratings are cookies, so the server
// renders them; the recipes a visitor writes are in localStorage (too big
// for cookies) and only exist in this browser. See docs/mock.md.

import { getContext, setContext } from 'svelte';
import type { Cookies } from 'crelte/cookies';

export type User = { name: string; email: string; bio: string };

export type Ingredient = { amount: number | null; unit: string; name: string };

export type Chapter = {
	title: string;
	ingredients: Ingredient[];
	// one paragraph per step
	steps: string[];
};

/** A recipe written in the editor, stored in this browser. */
export type LocalRecipe = {
	id: string;
	// the zipp recipe it changes, when it was edited from one
	basedOn: string | null;
	title: string;
	category: string | null;
	totalTime: number | null;
	activeTime: number | null;
	serves: number;
	chapters: Chapter[];
	postedAt: number;
};

const YEAR = 365 * 24 * 3600;
const KEY_RECIPES = 'chochbuech-recipes';
const KEY_DELETED = 'chochbuech-deleted';

export class Mock {
	user: User | null = $state(null);
	favorites: string[] = $state([]);
	ratings: Record<string, number> = $state({});
	// the weekly planner: {monday: recipeId, …}
	planner: Record<string, string> = $state({});
	// only on the client, after loadLocal()
	recipes: LocalRecipe[] = $state([]);
	deleted: string[] = $state([]);
	localLoaded = $state(false);

	private cookies: Cookies;

	constructor(cookies: Cookies) {
		this.cookies = cookies;
		this.user = parse(cookies.get('chochbuech-user'), null);
		this.favorites = parse(cookies.get('chochbuech-favorites'), []);
		this.ratings = parse(cookies.get('chochbuech-ratings'), {});
		this.planner = parse(cookies.get('chochbuech-planner'), {});
	}

	signIn(user: User) {
		this.user = user;
		this.save('chochbuech-user', user);
	}

	updateUser(user: User) {
		this.signIn(user);
	}

	signOut() {
		this.user = null;
		this.cookies.remove('chochbuech-user');
	}

	isFavorite(id: string): boolean {
		return this.favorites.includes(id);
	}

	toggleFavorite(id: string): boolean {
		const on = !this.isFavorite(id);
		this.favorites = on
			? [...this.favorites, id]
			: this.favorites.filter(f => f !== id);
		this.save('chochbuech-favorites', this.favorites);
		return on;
	}

	rate(id: string, stars: number) {
		this.ratings = { ...this.ratings, [id]: stars };
		this.save('chochbuech-ratings', this.ratings);
	}

	/** Plans a recipe for a day of the week, null clears the day. */
	plan(day: string, recipeId: string | null) {
		const next = { ...this.planner };
		if (recipeId) next[day] = recipeId;
		else delete next[day];
		this.planner = next;
		this.save('chochbuech-planner', next);
	}

	/** Reads the recipes of this browser, call it in an effect. */
	loadLocal() {
		this.recipes = parse(localStorage.getItem(KEY_RECIPES), []);
		this.deleted = parse(localStorage.getItem(KEY_DELETED), []);
		this.localLoaded = true;
	}

	localRecipe(id: string): LocalRecipe | null {
		return this.recipes.find(r => r.id === id) ?? null;
	}

	/** Saves a recipe of the editor, a new one gets an id. */
	saveRecipe(recipe: LocalRecipe): LocalRecipe {
		if (!recipe.id) recipe = { ...recipe, id: 'local-' + Date.now() };
		const others = this.recipes.filter(r => r.id !== recipe.id);
		this.recipes = [recipe, ...others];
		this.storeLocal();
		return recipe;
	}

	/** Deletes a local recipe, or hides a zipp recipe of the user. */
	deleteRecipe(id: string) {
		if (id.startsWith('local-')) {
			this.recipes = this.recipes.filter(r => r.id !== id);
		} else {
			this.deleted = [...this.deleted, id];
			// its edited copy goes too
			this.recipes = this.recipes.filter(r => r.basedOn !== id);
		}
		this.storeLocal();
	}

	private storeLocal() {
		localStorage.setItem(KEY_RECIPES, JSON.stringify(this.recipes));
		localStorage.setItem(KEY_DELETED, JSON.stringify(this.deleted));
	}

	private save(name: string, value: unknown) {
		this.cookies.set(name, JSON.stringify(value), { maxAge: YEAR });
	}
}

function parse<T>(s: string | null, fallback: T): T {
	if (!s) return fallback;
	try {
		return JSON.parse(s);
	} catch {
		return fallback;
	}
}

const KEY = Symbol('mock');

export function setMock(cookies: Cookies): Mock {
	return setContext(KEY, new Mock(cookies));
}

export function getMock(): Mock {
	return getContext(KEY);
}
