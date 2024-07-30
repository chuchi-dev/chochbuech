import { resolve, relative } from 'path';
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { sveltePreprocess } from 'svelte-preprocess';
// import { usedSsrComponents } from 'chuchi/ssr';

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
					runes: true,
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

// todo port this to chuchi/ssr
// maybe we need to do a new release with svelte 5
function usedSsrComponents(relativeFn) {
	return {
		transform(code, id, options) {
			if (!options?.ssr || !id.endsWith('.svelte')) return;

			// console.log('\n\n\n\nwhile building svelte', id, '\n', code);

			const file = relativeFn(id);

			const possibleSignatures = [
				'($$payload, $$props) {',
				'($$payload) {',
			];

			let idx = -1;
			for (const sig of possibleSignatures) {
				idx = code.indexOf(sig);
				if (idx >= 0) {
					idx += sig.length;
					break;
				}
			}
			if (idx < 0) {
				console.error('could not inject ssr modules tracking', id);
				return;
			}

			code = `
import { getContext as __modulesGetContext } from 'svelte';
${code.substr(0, idx)}
(() => {
const ctx = __modulesGetContext('modules');
if (ctx && ctx instanceof Set) {
	ctx.add('${file.replaceAll('\\', '\\\\')}');
}
})();
${code.substr(idx)}
`;

			return code;
		},
	};
}
