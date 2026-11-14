// The routes of the editor, on crelte's server: they forward to the cms's
// /members (zipp's plugin zipp-members, see lib/members.ts) with the
// token of the session cookie, which the browser never sees. Changes only
// take JSON from this site, the upload is the only multipart request.

import { error, fromThisSite, jsonFromThisSite } from './authRoutes.js';
import { SESSION_COOKIE } from './lib/siteAuth';

// the cms's limit of an image (`Members::images`) and room for the rest
// of the multipart body
const MAX_UPLOAD = 8 * 1024 * 1024 + 64 * 1024;

/** @param {import('crelte/server').ServerRouter} router */
export function memberRoutes(router) {
	router
		// the user's entries of a section
		.get('/members/entries', csr => {
			const search = new URL(csr.req.url).searchParams;
			const params = new URLSearchParams();
			for (const key of ['section', 'offset']) {
				const v = search.get(key);
				if (v) params.set(key, v);
			}
			return cms(csr, 'GET', '/members/entries', undefined, params);
		})
		.post('/members/entries', async csr => {
			const body = await jsonFromThisSite(csr);
			if (body instanceof Response) return body;
			return cms(csr, 'POST', '/members/entries', body);
		})
		.get('/members/entries/:id', csr =>
			withId(csr, id => cms(csr, 'GET', '/members/entries/' + id)),
		)
		.put('/members/entries/:id', csr =>
			withId(csr, async id => {
				const body = await jsonFromThisSite(csr);
				if (body instanceof Response) return body;
				return cms(csr, 'PUT', '/members/entries/' + id, body);
			}),
		)
		.delete('/members/entries/:id', csr =>
			withId(csr, id => {
				if (!fromThisSite(csr)) return error(403, 'Not allowed');
				return cms(csr, 'DELETE', '/members/entries/' + id);
			}),
		)
		.post('/members/assets', upload)
		// the images of a private recipe (cms/src/images.rs)
		.get('/members/images', csr => {
			const ids = new URL(csr.req.url).searchParams.get('ids') ?? '';
			if (!/^\d+(,\d+)*$/.test(ids)) return error(400, 'ids');
			const params = new URLSearchParams({ ids });
			return cms(csr, 'GET', '/members/images', undefined, params);
		})
		// a recipe of another site for the editor (cms/src/import)
		.post('/members/import', async csr => {
			const body = await jsonFromThisSite(csr);
			if (body instanceof Response) return body;
			return cms(csr, 'POST', '/members/import', { url: body.url });
		});
}

/**
 * @param {import('crelte/server').CrelteServerRequest} csr
 * @param {(id: string) => Promise<Response> | Response} fn
 */
function withId(csr, fn) {
	const id = csr.req.getParam('id') ?? '';
	if (!/^\d+$/.test(id)) return error(404, 'Not found');
	return fn(id);
}

/**
 * An image of the editor: multipart, read up to the limit, then
 * forwarded as it came.
 *
 * @param {import('crelte/server').CrelteServerRequest} csr
 */
async function upload(csr) {
	const type = csr.req.headers.get('content-type') ?? '';
	if (!type.startsWith('multipart/form-data') || !fromThisSite(csr)) {
		return error(403, 'Not allowed');
	}
	const tooBig = () => error(413, 'The image is too big, at most 8 MB.');
	const length = Number(csr.req.headers.get('content-length') ?? 0);
	if (length > MAX_UPLOAD) return tooBig();
	if (!csr.req.body) return error(400, 'No image');

	/** @type {Uint8Array<ArrayBuffer>[]} */
	const chunks = [];
	let size = 0;
	const reader = csr.req.body.getReader();
	for (;;) {
		const { done, value } = await reader.read();
		if (done) break;
		size += value.length;
		if (size > MAX_UPLOAD) {
			await reader.cancel();
			return tooBig();
		}
		chunks.push(/** @type {Uint8Array<ArrayBuffer>} */ (value));
	}
	// as it came: the type has the boundary (a Blob's type is lower case)
	return cms(csr, 'POST', '/members/assets', new Blob(chunks), null, type);
}

/**
 * A request to the cms with the session's token; its answer goes to the
 * browser as it is (JSON, the errors with their field paths).
 *
 * @param {import('crelte/server').CrelteServerRequest} csr
 * @param {string} method
 * @param {string} path
 * @param {unknown} [body] JSON, or a Blob of `type`
 * @param {URLSearchParams | null} [params]
 * @param {string} [type]
 */
async function cms(csr, method, path, body, params, type) {
	const token = csr.cookies.get(SESSION_COOKIE);
	if (!token) return error(401, 'Please sign in first.');
	/** @type {Record<string, string>} */
	const headers = { authorization: `Bearer ${token}` };
	/** @type {BodyInit | undefined} */
	let data;
	if (body instanceof Blob) {
		headers['content-type'] = type ?? 'application/octet-stream';
		data = body;
	} else if (body !== undefined) {
		headers['content-type'] = 'application/json';
		data = JSON.stringify(body);
	}
	const url = csr.backendUrl(path);
	if (params) url.search = params.toString();
	let resp;
	try {
		resp = await fetch(url, { method, headers, body: data });
	} catch (e) {
		console.error('the cms does not answer', e);
		return error(502, 'The server could not answer, try again later.');
	}
	const text = await resp.text();
	const out = new Response(text || 'null', {
		status: resp.status,
		headers: { 'content-type': 'application/json' },
	});
	out.headers.set('cache-control', 'no-store');
	return out;
}
