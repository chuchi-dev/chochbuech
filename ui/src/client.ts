import App from './App.svelte';
import * as routes from './pages/routes';
import { handleRoute } from './main';
import { SsrCache } from 'chuchi/ssr';
import { Router } from 'chuchi';

async function main() {
	const cache = new SsrCache();
	const router = new Router();

	const context = new Map();
	context.set('router', router);

	routes.register(router);

	let app: App | null;

	router.onRoute(async (req, route, routing) => {
		const { props } = await handleRoute(req, route, cache);

		if (await routing.dataReady()) return;

		if (!app) {
			app = new App({
				target: document.body,
				// @ts-ignore
				props,
				hydrate: true,
				context,
			});
		} else {
			app.$set(props);
		}

		routing.domReady();
	});

	router.initClient();
}
main();
