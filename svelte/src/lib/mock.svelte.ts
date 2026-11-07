// What a visitor does is a mock for now, kept by their browser: logging in
// (lib/session.svelte.ts) and writing recipes (lib/members.ts) are real,
// but zipp has no favorites, ratings or planner yet. They are cookies, so
// the server renders them. See docs/mock.md.

import { getContext, setContext } from 'svelte';
import type { Cookies } from 'crelte/cookies';

const YEAR = 365 * 24 * 3600;

export class Mock {
	favorites: string[] = $state([]);
	ratings: Record<string, number> = $state({});
	// the weekly planner: {monday: recipeId, …}
	planner: Record<string, string> = $state({});

	private cookies: Cookies;

	constructor(cookies: Cookies) {
		this.cookies = cookies;
		this.favorites = parse(cookies.get('chochbuech-favorites'), []);
		this.ratings = parse(cookies.get('chochbuech-ratings'), {});
		this.planner = parse(cookies.get('chochbuech-planner'), {});
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
