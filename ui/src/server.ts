import { render as renderComponent } from 'svelte/server';
import App from './App.svelte';
import * as routes from './pages/routes';
import { handleRoute } from './main';
import { SsrCache, SsrComponents } from 'chuchi/ssr';
import { Router } from 'chuchi';
import { Writable } from 'chuchi/stores';
import { ServerCookies } from 'chuchi/cookies';
import Session from './lib/Session';
import LoadProps from './lib/LoadProps';

// req: { method, uri, ?ssrManifest, cookies }
// opt: { ssrManifest }
// returns: { status, body, head, setCookies }
export async function render(req: any, opt: any) {
	const cache = new SsrCache();
	const router = new Router();
	const cookies = new ServerCookies();
	const ssrComponents = new SsrComponents();

	const context = new Map();
	context.set('router', router);
	context.set('cookies', cookies);
	ssrComponents.addToContext(context);

	routes.register(router);

	req = router.initServer('http://' + req.headers.host + req.uri);
	cookies._init(req.cookies ?? '');

	const session = await Session.init(cache, cookies);
	context.set('session', session);

	const route = router.route(req);
	const loadProps = new LoadProps({
		router,
		route,
		req,
		cookies,
		cache,
		session,
	});
	const { status, page, redirect } = await handleRoute(req, route, loadProps);

	if (redirect) throw new Error('redirect ' + redirect);

	const pageStore = new Writable(page);

	let { html, head } = renderComponent(App, {
		props: {
			page: pageStore,
		},
		context,
	});

	head += ssrComponents.toHead(opt?.ssrManifest ?? {});
	head += cache.toHead();

	return {
		status,
		fields: {
			head,
			body: html,
		},
		setCookies: cookies._getSetCookiesHeaders(),
	};
}

export { routes };
