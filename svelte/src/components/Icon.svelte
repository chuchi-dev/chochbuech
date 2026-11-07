<script module lang="ts">
	// Lucide icons (ISC licence), drawn with currentColor
	const ICONS = {
		heart: '<path d="M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5"/>',
		star: '<path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/>',
		sliders:
			'<path d="M10 5H3"/><path d="M12 19H3"/><path d="M14 3v4"/><path d="M16 17v4"/><path d="M21 12h-9"/><path d="M21 19h-5"/><path d="M21 5h-7"/><path d="M8 10v4"/><path d="M8 12H3"/>',
		right: '<path d="M5 12h14"/><path d="m12 5 7 7-7 7"/>',
		left: '<path d="m12 19-7-7 7-7"/><path d="M19 12H5"/>',
		up: '<path d="m5 12 7-7 7 7"/><path d="M12 19V5"/>',
		down: '<path d="M12 5v14"/><path d="m19 12-7 7-7-7"/>',
		minus: '<path d="M5 12h14"/>',
		plus: '<path d="M5 12h14"/><path d="M12 5v14"/>',
		x: '<path d="M18 6 6 18"/><path d="m6 6 12 12"/>',
		move: '<path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/>',
		thumbsUp:
			'<path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2a3.13 3.13 0 0 1 3 3.88Z"/><path d="M7 10v12"/>',
		thumbsDown:
			'<path d="M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22a3.13 3.13 0 0 1-3-3.88Z"/><path d="M17 14V2"/>',
	};

	export type IconName = keyof typeof ICONS;
</script>

<script lang="ts">
	let {
		name,
		filled = false,
		size = '1em',
	}: { name: IconName; filled?: boolean; size?: string } = $props();
</script>

<svg
	viewBox="0 0 24 24"
	width={size}
	height={size}
	fill={filled ? 'currentColor' : 'none'}
	stroke="currentColor"
	stroke-width="1.5"
	stroke-linecap="round"
	stroke-linejoin="round"
	aria-hidden="true"
>
	{@html ICONS[name]}
</svg>
