// The routes the browser logs in with, on crelte's server: they forward
// to the cms's /site-auth (see lib/siteAuth.ts) and keep the session
// token in the httpOnly cookie; the browser only gets `{ user }`.

import { SESSION_COOKIE, siteAuth } from './lib/siteAuth';

/** @param {import('crelte/server').ServerRouter} router */
export function authRoutes(router) {
	router
		.post('/auth/login', csr =>
			post(csr, async (body, ip) => {
				const req = {
					email: str(body.email),
					password: str(body.password),
				};
				return login(csr, await cms(csr, 'login', { body: req, ip }));
			}),
		)
		// an invite or reset link: sets the password and logs in
		.post('/auth/password', csr =>
			post(csr, async (body, ip) => {
				const req = {
					token: str(body.token),
					password: str(body.password),
				};
				return login(
					csr,
					await cms(csr, 'password', { body: req, ip }),
				);
			}),
		)
		// mails a link to /reset-password, the same answer for any email
		.post('/auth/reset', csr =>
			post(csr, async (body, ip) => {
				const req = { email: str(body.email), site: csr.site.handle };
				return answer(await cms(csr, 'reset', { body: req, ip }), {});
			}),
		)
		.post('/auth/logout', csr =>
			post(csr, async () => {
				const token = csr.cookies.get(SESSION_COOKIE);
				if (token) await cms(csr, 'logout', { token, body: {} });
				return withCookie(Response.json({}), csr, '', 0);
			}),
		)
		// the user's own profile: their name, bio and photo (an image of
		// /members/assets); the cms answers with the user
		.put('/auth/me', csr =>
			post(csr, async body => {
				const token = csr.cookies.get(SESSION_COOKIE);
				if (!token) return error(401, 'Please sign in first.');
				const fields = /** @type {Record<string, unknown>} */ (
					body.fields && typeof body.fields === 'object'
						? body.fields
						: {}
				);
				const req = {
					name: str(body.name),
					fields: {
						bio: str(fields.bio),
						photo: Array.isArray(fields.photo)
							? fields.photo.filter(Number.isInteger).slice(0, 1)
							: [],
					},
				};
				const resp = await cms(csr, 'me', {
					token,
					body: req,
					method: 'PUT',
				});
				if (!resp.ok) return answer(resp, null);
				return Response.json({ user: await resp.json() });
			}),
		)
		// the user of the session, renews it (and the cookie)
		.get('/auth/me', async csr => {
			const token = csr.cookies.get(SESSION_COOKIE);
			if (!token) return Response.json({ user: null });
			const resp = await cms(csr, 'me', { token });
			if (resp.status === 401 || resp.status === 403) {
				return withCookie(Response.json({ user: null }), csr, '', 0);
			}
			return login(csr, resp);
		});
}

/**
 * @param {import('crelte/server').CrelteServerRequest} csr
 * @param {string} path
 * @param {{ token?: string, body?: unknown, ip?: string | null, method?: string }} opts
 */
async function cms(csr, path, opts) {
	try {
		return await siteAuth(p => csr.backendUrl(p), path, opts);
	} catch (e) {
		console.error('the cms does not answer', e);
		return error(502, 'The server could not answer, try again later.');
	}
}

/**
 * A POST of the browser: only JSON (a form of another site can't send
 * that without CORS) and only from this site, against CSRF.
 *
 * @param {import('crelte/server').CrelteServerRequest} csr
 * @param {(body: Record<string, unknown>, ip: string | null) => Promise<Response>} fn
 */
async function post(csr, fn) {
	const body = await jsonFromThisSite(csr);
	if (body instanceof Response) return body;
	return fn(body, visitorIp(csr));
}

/**
 * Whether a request that changes something comes from this site: the
 * browser sends the origin, a form or script of another site can't fake
 * it (against CSRF).
 *
 * @param {import('crelte/server').CrelteServerRequest} csr
 */
export function fromThisSite(csr) {
	const origin = csr.req.headers.get('origin');
	const origins = [csr.frontendUrl(), ...csr.sites.map(s => s.url)];
	return origins.some(u => u.origin === origin);
}

/**
 * The JSON object of a request from this site, or the error to answer.
 *
 * @param {import('crelte/server').CrelteServerRequest} csr
 * @returns {Promise<Record<string, unknown> | Response>}
 */
export async function jsonFromThisSite(csr) {
	const type = csr.req.headers.get('content-type') ?? '';
	if (!type.startsWith('application/json') || !fromThisSite(csr)) {
		return error(403, 'Not allowed');
	}
	/** @type {unknown} */
	let body;
	try {
		body = await csr.req.json();
	} catch {
		return error(400, 'The request is no JSON');
	}
	if (!body || typeof body !== 'object') return error(400, 'Not an object');
	return /** @type {Record<string, unknown>} */ (body);
}

/**
 * The visitor's address: the proxy in front of this server (Caddy) adds
 * it at the end of X-Forwarded-For, what the visitor wrote before it
 * doesn't count. crelte gives no socket address.
 *
 * @param {import('crelte/server').CrelteServerRequest} csr
 */
function visitorIp(csr) {
	const hops = csr.req.headers.get('x-forwarded-for')?.split(',') ?? [];
	return hops.at(-1)?.trim() || null;
}

/**
 * A login of the cms (login, password, me): the token goes into the
 * cookie, the browser gets the user.
 *
 * @param {import('crelte/server').CrelteServerRequest} csr
 * @param {Response} resp
 */
async function login(csr, resp) {
	if (!resp.ok) return answer(resp, null);
	/** @type {import('./lib/siteAuth').SiteAuth} */
	const auth = await resp.json();
	const maxAge = Math.floor((auth.session.expiresAt - Date.now()) / 1000);
	const user = auth.user;
	return withCookie(Response.json({ user }), csr, auth.session.token, maxAge);
}

/**
 * The cms's answer for the browser: `ok` when it worked, else its error
 * (`{ message, error }`).
 *
 * @param {Response} resp
 * @param {unknown} ok
 */
export async function answer(resp, ok) {
	if (resp.ok) return Response.json(ok);
	const text = await resp.text();
	let body;
	try {
		body = JSON.parse(text);
	} catch {
		body = { message: 'The server could not answer, try again later.' };
	}
	return Response.json(body, { status: resp.status });
}

/** @param {number} status @param {string} message */
export function error(status, message) {
	return Response.json({ message }, { status });
}

/**
 * The session cookie: httpOnly, SameSite=Lax, Secure over https. crelte's
 * cookies have no SameSite, so it is set here.
 *
 * @param {Response} resp
 * @param {import('crelte/server').CrelteServerRequest} csr
 * @param {string} token
 * @param {number} maxAge
 */
function withCookie(resp, csr, token, maxAge) {
	const secure = csr.frontendUrl().protocol === 'https:' ? '; Secure' : '';
	resp.headers.append(
		'set-cookie',
		`${SESSION_COOKIE}=${encodeURIComponent(token)}; Max-Age=${maxAge}` +
			`; Path=/; HttpOnly; SameSite=Lax${secure}`,
	);
	resp.headers.set('cache-control', 'no-store');
	return resp;
}

/** @param {unknown} v */
function str(v) {
	return typeof v === 'string' ? v : '';
}
