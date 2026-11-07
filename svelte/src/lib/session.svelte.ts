// Who is logged in, a zipp user (see lib/siteAuth.ts). The token is in an
// httpOnly cookie the browser can't read: the server render loads the user
// (loadSession, in App's loadGlobalData) and hands it to the browser in
// crelte's SsrCache; after that the browser keeps it and changes it
// through the routes of authRoutes.js.

import { getContext, setContext } from 'svelte';
import type { CrelteRequest } from 'crelte';
import { SESSION_COOKIE, siteAuth, type User } from './siteAuth';

export type { User };

const SSR_KEY = 'chochbuech-user';

/** A failed request of the session, `message` is for the visitor. */
export class AuthError extends Error {
	status: number;
	// zipp's error type, e.g. LOGIN_INCORRECT, REQUEST, INVALID
	type: string | null;

	constructor(status: number, message: string, type: string | null) {
		super(message);
		this.status = status;
		this.type = type;
	}
}

/** What to tell the visitor about a failed request. */
export function errorMessage(e: unknown): string {
	if (e instanceof AuthError) return e.message;
	return 'The server could not answer, try again later.';
}

export class Session {
	user: User | null = $state(null);

	constructor(user: User | null) {
		this.user = user;
	}

	async login(email: string, password: string): Promise<User> {
		const { user } = await call('login', { email, password });
		this.user = user;
		return user;
	}

	/** Sets the password of an invite or reset link and logs in. */
	async setPassword(token: string, password: string): Promise<User> {
		const { user } = await call('password', { token, password });
		this.user = user;
		return user;
	}

	/** Mails a link to set a new password, if the email has an account. */
	async requestReset(email: string): Promise<void> {
		await call('reset', { email });
	}

	async logout(): Promise<void> {
		await call('logout', {});
		this.user = null;
	}

	/** Changes the user's own profile, the fields it sends. */
	async updateProfile(
		name: string,
		fields: { bio: string; photo: number[] },
	): Promise<User> {
		const { user } = await call('me', { name, fields }, 'PUT');
		this.user = user;
		return user;
	}

	/** Reads the user again, renews the session and its cookie. */
	async refresh(): Promise<void> {
		const { user } = await call('me');
		this.user = user;
	}
}

async function call(path: string, body?: unknown, method?: string) {
	const resp = await fetch('/auth/' + path, {
		method: method ?? (body === undefined ? 'GET' : 'POST'),
		headers:
			body === undefined ? {} : { 'content-type': 'application/json' },
		body: body === undefined ? undefined : JSON.stringify(body),
	});
	const data = await resp.json().catch(() => ({}));
	if (resp.ok) return data;
	// zipp's message, without its "Request error: "; a password too short
	// is a field error
	const type = data.error?.type ?? null;
	const detail = data.error?.detail;
	const message =
		(type === 'REQUEST' && detail) ||
		(type === 'INVALID' && detail?.[0]?.message) ||
		data.message ||
		'Something went wrong, try again.';
	throw new AuthError(resp.status, message, type);
}

/**
 * The user of the session cookie for the server render, in the SsrCache
 * for the browser. The cms renews the session here, the browser renews
 * the cookie after it started (crelte's cookies can't set SameSite).
 */
export async function loadSession(cr: CrelteRequest): Promise<void> {
	const token = cr.cookies.get(SESSION_COOKIE);
	let user: User | null = null;
	if (token) {
		try {
			const resp = await siteAuth(p => cr.backendUrl(p), 'me', { token });
			if (resp.ok) user = (await resp.json()).user;
			else if (resp.status === 401 || resp.status === 403) {
				// ended, logged out elsewhere or a new password
				cr.cookies.remove(SESSION_COOKIE);
			}
		} catch (e) {
			console.error('the cms does not answer', e);
		}
	}
	cr.ssrCache.set(SSR_KEY, user);
}

const KEY = Symbol('session');

/** The session of the app, with the user of the server render. */
export function setSession(cr: { ssrCache: CrelteRequest['ssrCache'] }) {
	return setContext(KEY, new Session(cr.ssrCache.get<User>(SSR_KEY)));
}

export function getSession(): Session {
	return getContext(KEY);
}
