<script module>
	import { createStaticRouter, getStaticRouter } from 'crelte/static';
	import { staticRoutes } from './routes.js';
	import { loadSession } from './lib/session.svelte';
	import globalQuery from './queries/global.graphql';

	export const templates = import.meta.glob('@/templates/*.svelte');

	export const plugins = [createStaticRouter()];

	/** @param {import('crelte').Crelte} crelte */
	export function init(crelte) {
		staticRoutes(getStaticRouter(crelte));
	}

	// the globals (header, footer, categories) and the user of the session
	// cookie, the latter only for the server render: the browser takes it
	// from there (lib/session.svelte.ts)
	/** @param {import('crelte').CrelteRequest} cr */
	export async function loadGlobalData(cr) {
		const [globals] = await Promise.all([
			cr.query(globalQuery),
			import.meta.env.SSR ? loadSession(cr) : null,
		]);
		return globals;
	}

	/** @type {import('crelte').Config} */
	export const config = {
		preloadOnMouseOver: true,
	};
</script>

<script>
	import { onMount } from 'svelte';
	import { getCookies, getCrelte } from 'crelte';
	import { Seo } from 'meierlabs-seo';
	import Footer from './components/Footer.svelte';
	import Header from './components/Header.svelte';
	import Toasts from './components/Toasts.svelte';
	import { setMock } from './lib/mock.svelte';
	import { setSession } from './lib/session.svelte';
	import { setToasts } from './lib/toasts.svelte';

	const session = setSession(getCrelte());
	setMock(getCookies());
	setToasts();

	/** @type {import('crelte').AppProps} */
	let { route } = $props();

	let entry = $derived($route.entry);
	let Template = $derived($route.template.default);
	let templateData = $derived($route.loadedData);

	// renews the session's cookie, the server render can't
	onMount(() => {
		if (session.user) session.refresh().catch(() => {});
	});
</script>

<Seo seo={entry?.seoJson} />

<Header />

<!-- update entire component if page changes -->
{#key entry?.url ?? $route.url.href}
	<main class="app">
		<Template {entry} {...templateData} />
	</main>
{/key}

<Footer />
<Toasts />

<style lang="scss">
	.app {
		min-height: calc(100vh - var(--topbar-height) - 5rem);
	}
</style>
