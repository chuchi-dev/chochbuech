import {
	Authenticated,
	tokenAuth,
	type ShortSession,
	type ShortUser,
} from '@/api/users';
import Listeners from 'chuchi-utils/sync/Listeners';
import type { Cookies } from 'chuchi/cookies';
import type { SsrCache } from 'chuchi/ssr';
import { getContext } from 'svelte';

/**
 * ## Note
 * If you use this in a component use it with $session to make sure you
 * get updated if someting changes
 */
export default class Session {
	// if shortSession is defined shortUser is defined as well
	shortSession: ShortSession | null;
	shortUser: ShortUser | null;
	listeners: Listeners<[]>;

	constructor() {
		this.shortSession = null;
		this.shortUser = null;
		this.listeners = new Listeners();
	}

	// will not throw
	static async init(cache: SsrCache, cookies: Cookies): Promise<Session> {
		const me = new Session();

		// if we are on the client the server as already executed
		// this code and we can just get the data from the cache
		if (!import.meta.env.SSR) {
			const data = cache.get('session');
			me.shortSession = data.shortSession;
			me.shortUser = data.shortUser;

			return me;
		}

		const auth = await authed(cookies);
		if (auth) {
			me.shortSession = auth.session;
			me.shortUser = auth.user;
		}

		cache.set('session', {
			shortSession: me.shortSession,
			shortUser: me.shortUser,
		});

		return me;
	}

	subscribe(fn: (sess: Session) => void): () => void {
		return this.listeners.add(() => fn(this));
	}

	setAuthed(auth: Authenticated) {
		this.shortSession = auth.session;
		this.shortUser = auth.user;
		this.listeners.trigger();
	}
}

async function authed(cookies: Cookies): Promise<Authenticated | null> {
	const sessionToken = cookies.get('SESSION_TOKEN');
	if (!sessionToken) return null;

	try {
		return await tokenAuth(sessionToken);
	} catch (e) {
		console.error('Error getting session', e);
		return null;
	}
}

export function getSession(): Session {
	return getContext('session');
}
