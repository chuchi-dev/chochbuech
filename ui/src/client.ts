import App from './App.svelte';
import * as routes from './pages/routes';
import { handleRoute } from './main';
import { SsrCache } from 'chuchi/ssr';
import { Router } from 'chuchi';
import { hydrate, mount, tick } from 'svelte';
import { Writable } from 'chuchi/stores';

async function main() {
	const cache = new SsrCache();
	const router = new Router();

	const context = new Map();
	context.set('router', router);

	routes.register(router);

	let hydrated = false;
	let pageStore = new Writable<any>(null);

	router.onRoute(async (req, route, routing) => {
		const { page } = await handleRoute(req, route, cache);

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
