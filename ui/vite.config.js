import { resolve, relative } from 'path';
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import sveltePreprocess from 'svelte-preprocess';
import { usedSsrComponents } from 'chuchi/ssr';

// https://vitejs.dev/config/
export default defineConfig(({ isSsrBuild }) => {
	return {
		publicDir: isSsrBuild ? false : undefined,
		ssr: {
			noExternal: isSsrBuild ? true : ['chuchi'],
		},
		plugins: [
			svelte({
				compilerOptions: {
					hydratable: true,
				},
				preprocess: [sveltePreprocess()],
			}),
			usedSsrComponents(f => relative(__dirname, f)),
		],
		resolve: {
			alias: [{ find: '@', replacement: resolve(__dirname, 'src') }],
		},
	};
});
