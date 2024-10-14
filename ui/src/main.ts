import type { Request, Route, Router } from 'chuchi';
import * as routes from './pages/routes';
import type { SsrCache } from 'chuchi/ssr';
import { getContext } from 'svelte';
import type { Cookies } from 'chuchi/cookies';
import type LoadProps from './lib/LoadProps';

export function getRouter(): Router {
	return getContext('router');
}

export function getCookies(): Cookies {
	return getContext('cookies');
}

export type RouteResponse = {
	status: number;
	page?: any;
	redirect?: string;
};

// should return { status, props,  }
export async function handleRoute(
	req: Request,
	route: Route | null,
	loadProps: LoadProps,
): Promise<RouteResponse> {
	if (route) {
		let comp, pageProps;
		try {
			comp = await route.load(req);

			if (comp?.requiresLogin && !loadProps.session.isLoggedIn()) {
				return {
					status: 302,
					redirect:
						'/signin?' +
						new URLSearchParams([
							['url', req.url.pathname],
						]).toString(),
				};
			}

			if (typeof comp.loadProps === 'function')
				pageProps = await comp.loadProps(route.toProps(req), loadProps);
			else pageProps = {};

			if (loadProps.redirect) {
				return {
					status: loadProps.redirect.status,
					redirect: loadProps.redirect.url,
				};
			}
		} catch (e) {
			console.log('error', e);
			return {
				status: 500,
				page: {
					component: routes.NotFound,
					props: {},
				},
			};
		}

		return {
			status: 200,
			page: {
				component: comp.default,
				props: pageProps,
			},
		};
	}

	return {
		status: 404,
		page: {
			component: routes.NotFound,
			props: {},
		},
	};
}
