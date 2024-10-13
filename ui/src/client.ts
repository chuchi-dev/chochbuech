import App from './App.svelte';
import * as routes from './pages/routes';
import { handleRoute } from './main';
import { SsrCache } from 'chuchi/ssr';
import { Router } from 'chuchi';
import { hydrate, mount, tick } from 'svelte';
import { Writable } from 'chuchi/stores';
import { ClientCookies } from 'chuchi/cookies';
import Session from './lib/Session';
import LoadProps from './lib/LoadProps';

async function main() {
	const cache = new SsrCache();
	const router = new Router();
	const cookies = new ClientCookies();

	const context = new Map();
	context.set('router', router);
	context.set('cookies', cookies);

	routes.register(router);

	const session = await Session.init(cache, cookies);
	context.set('session', session);

	let hydrated = false;
	let pageStore = new Writable<any>(null);

	router.onRoute(async (req, route, routing) => {
		const loadProps = new LoadProps({
			router,
			route,
			req,
			cookies,
			cache,
			session,
		});
		const { page, redirect } = await handleRoute(req, route, loadProps);

		if (redirect) {
			// todo handle the request?
			router.open(redirect);
			return;
		}

		if (await routing.dataReady()) return;

		pageStore.set(page);

		if (!hydrated) {
			hydrated = true;
			hydrate(App, {
				target: document.body,
				// @ts-ignore
				props: { page: pageStore },
				context,
			});
		}

		await tick();

		routing.domReady();
	});

	router.initClient();
}
main();
