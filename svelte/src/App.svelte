<script module>
	import { createStaticRouter, getStaticRouter } from 'crelte/static';
	import { staticRoutes } from './routes.js';

	export const templates = import.meta.glob('@/templates/*.svelte');

	export const plugins = [createStaticRouter()];

	/** @param {import('crelte').Crelte} crelte */
	export function init(crelte) {
		staticRoutes(getStaticRouter(crelte));
	}

	/** @type {import('crelte').Config} */
	export const config = {
		preloadOnMouseOver: true,
	};
</script>

<script>
	import { getCookies } from 'crelte';
	import { Seo } from 'meierlabs-seo';
	import Footer from './components/Footer.svelte';
	import Header from './components/Header.svelte';
	import Toasts from './components/Toasts.svelte';
	import { setMock } from './lib/mock.svelte';
	import { setToasts } from './lib/toasts.svelte';

	const mock = setMock(getCookies());
	setToasts();

	/** @type {import('crelte').AppProps} */
	let { route } = $props();

	let entry = $derived($route.entry);
	let Template = $derived($route.template.default);
	let templateData = $derived($route.loadedData);

	// the recipes written in this browser
	$effect(() => {
		mock.loadLocal();
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
