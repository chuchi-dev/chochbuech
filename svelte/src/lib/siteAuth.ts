// The logins of the website are zipp's (its docs/members.md): the crelte
// server calls the cms's /site-auth routes server to server and keeps the
// session token in an httpOnly cookie, the browser never sees it. The
// routes the browser calls are in authRoutes.js, the user of a page's
// first render is loaded in loadSession (lib/session.svelte.ts).

export const SESSION_COOKIE = 'chochbuech-session';

export type User = {
	id: number;
	name: string;
	email: string;
	// null is an admin of the studio
	role: string | null;
	// of their page `/cooks/<slug>`, which they have while a recipe of
	// theirs is live
	slug: string;
	// the user fields of the cms: bio, photo (asset ids), memberSince
	fields: { bio?: string; photo?: number[]; memberSince?: number };
};

/** The answer of login, me and password: the session and its user. */
export type SiteAuth = {
	session: { token: string; expiresAt: number };
	user: User;
};

/**
 * A request to the cms's `/site-auth/<path>`, a POST with a body. `ip` is
 * the visitor's address for the cms's limit of wrong passwords, the cms
 * trusts it from this server (`serve --trusted-proxy`).
 */
export function siteAuth(
	backend: (path: string) => URL,
	path: string,
	opts: {
		token?: string | null;
		body?: unknown;
		ip?: string | null;
		method?: string;
	} = {},
): Promise<Response> {
	const headers: Record<string, string> = {};
	if (opts.token) headers['authorization'] = `Bearer ${opts.token}`;
	if (opts.ip) headers['x-forwarded-for'] = opts.ip;
	if (opts.body !== undefined) headers['content-type'] = 'application/json';
	return fetch(backend('/site-auth/' + path), {
		method: opts.method ?? (opts.body === undefined ? 'GET' : 'POST'),
		headers,
		body: opts.body === undefined ? undefined : JSON.stringify(opts.body),
	});
}
