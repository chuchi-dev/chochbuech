import type { Route, Router, Request } from 'chuchi';
import type { Cookies } from 'chuchi/cookies';
import type { SsrCache } from 'chuchi/ssr';
import type Session from './Session';

export default class LoadProps {
	router: Router;
	route: Route | null;
	req: Request;
	cookies: Cookies;
	cache: SsrCache;
	session: Session;
	redirect: { status: number; url: string } | null;

	constructor(obj: {
		router: Router;
		route: Route | null;
		req: Request;
		cookies: Cookies;
		cache: SsrCache;
		session: Session;
	}) {
		this.router = obj.router;
		this.route = obj.route;
		this.req = obj.req;
		this.cookies = obj.cookies;
		this.cache = obj.cache;
		this.session = obj.session;
		this.redirect = null;
	}

	setRedirect(url: string, status = 302) {
		this.redirect = { status, url };
	}
}
