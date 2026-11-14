// The recipes of members: zipp entries with the member as author, written
// in the editor through zipp's plugin zipp-members (its docs/members.md).
// The browser calls the routes of memberRoutes.js, which add the session
// token. Everybody edits only the recipes they wrote, admins too. An
// unpublished recipe is a private one: only its author sees it.

/** An entry as zipp-members gives it, with its draft if it has one. */
export type MemberEntry = {
	id: number;
	section: string;
	type: string;
	site: string;
	title: string;
	slug: string;
	uri: string | null;
	// the page on the website
	url: string | null;
	published: boolean;
	// ms
	postDate: number | null;
	fields: Record<string, any>;
	hasDraft: boolean;
};

export type Uploaded = {
	id: number;
	url: string;
	width: number | null;
	height: number | null;
};

/** A field that zipp refused, `path` like `chapters.0.title`. */
export type FieldError = { path: string; message: string };

/** A failed request, `message` is for the visitor. */
export class MemberError extends Error {
	status: number;
	fields: FieldError[];

	constructor(status: number, message: string, fields: FieldError[] = []) {
		super(message);
		this.status = status;
		this.fields = fields;
	}
}

/** The recipes of the user, the newest first. */
export async function listRecipes(): Promise<{
	entries: MemberEntry[];
	total: number;
}> {
	return call('GET', '/members/entries?section=recipes');
}

export function getEntry(id: string | number): Promise<MemberEntry> {
	return call('GET', `/members/entries/${id}`);
}

/** A new recipe, live at once when `published`, else private. */
export function createRecipe(
	title: string,
	fields: Record<string, unknown>,
	published: boolean,
): Promise<MemberEntry> {
	return call('POST', '/members/entries', {
		section: 'recipes',
		type: 'recipe',
		title,
		fields,
		published,
	});
}

/** Changes the fields it sends and publishes or unpublishes it; `saved`
 * is `draft` if the role may not publish. */
export function updateEntry(
	id: number,
	title: string,
	fields: Record<string, unknown>,
	published: boolean,
): Promise<{ saved: 'live' | 'draft'; entry: MemberEntry }> {
	return call('PUT', `/members/entries/${id}`, { title, fields, published });
}

/** A recipe of another site as the cms's import reads it (cms/src/import):
 * nothing is saved, it fills the editor. */
export type ImportedRecipe = {
	title: string;
	// the slug of a category
	category: string | null;
	serves: number | null;
	// minutes
	totalTime: number | null;
	activeTime: number | null;
	chapters: {
		title: string;
		ingredients: {
			amount: number | null;
			unit: string | null;
			name: string;
		}[];
		steps: string[];
	}[];
	// the page it came from
	source: string;
};

/** Reads the recipe of the page at `url`: the cms fetches that page and a
 * model writes it in English and its own words. */
export function importRecipe(url: string): Promise<ImportedRecipe> {
	return call('POST', '/members/import', { url });
}

/** Publishes a recipe (zipp checks its photo and the other required
 * fields) or makes it private again. */
export function setPublished(
	e: MemberEntry,
	published: boolean,
): Promise<{ saved: 'live' | 'draft'; entry: MemberEntry }> {
	return updateEntry(e.id, e.title, {}, published);
}

/** The images of these ids, for a private recipe: GraphQL only has the
 * live ones. */
export async function getImages(ids: number[]): Promise<Uploaded[]> {
	if (!ids.length) return [];
	return call('GET', `/members/images?ids=${ids.join(',')}`);
}

export function deleteEntry(id: number): Promise<void> {
	return call('DELETE', `/members/entries/${id}`);
}

/** An image for an image field. */
export async function uploadImage(file: File): Promise<Uploaded> {
	const data = new FormData();
	data.append('file', file);
	const resp = await fetch('/members/assets', { method: 'POST', body: data });
	return answer(resp);
}

async function call(method: string, path: string, body?: unknown) {
	let resp;
	try {
		resp = await fetch(path, {
			method,
			headers:
				body === undefined
					? {}
					: { 'content-type': 'application/json' },
			body: body === undefined ? undefined : JSON.stringify(body),
		});
	} catch {
		throw new MemberError(0, 'The server could not answer, try again.');
	}
	return answer(resp);
}

async function answer(resp: Response) {
	const data = await resp.json().catch(() => null);
	if (resp.ok) return data;
	const err = data?.error;
	const fields: FieldError[] = err?.type === 'INVALID' ? err.detail : [];
	let message: string;
	if (resp.status === 401) message = 'Please sign in first.';
	else if (resp.status === 403)
		message = 'This recipe is not yours, you cannot change it.';
	else if (resp.status === 404) message = 'This recipe does not exist.';
	else if (resp.status === 413)
		message = 'The image is too big, at most 8 MB.';
	else if (fields.length) message = 'Please check the marked fields.';
	else
		message =
			(err?.type === 'REQUEST' && typeof err.detail === 'string'
				? err.detail
				: data?.message) || 'Something went wrong, try again.';
	throw new MemberError(resp.status, message, fields);
}

/** What to tell the member when publishing failed. */
export function publishError(e: unknown): string {
	if (!(e instanceof MemberError) || !e.fields.length) return errorMessage(e);
	return e.fields.some(f => f.path.startsWith('image'))
		? 'Add your own photo to publish.'
		: 'Please complete the recipe in the editor to publish it.';
}

/** What to tell the visitor about a failed request. */
export function errorMessage(e: unknown): string {
	if (e instanceof MemberError) return e.message;
	return 'Something went wrong, try again.';
}

/**
 * Forgets the answers of the queries: crelte keeps them in the browser
 * (its SsrCache) for the rest of the visit, so after a change the pages
 * would show the recipe as it was.
 */
export function forgetQueries(crelte: { ssrCache: { z_clear(): void } }) {
	crelte.ssrCache.z_clear();
}
