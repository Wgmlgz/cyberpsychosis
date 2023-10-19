<script lang="ts">
	import { onMount } from 'svelte';
	import { WebviewWindow } from '@tauri-apps/api/window';

	let amogus: HTMLImageElement;

	onMount(() => {
		const webview = new WebviewWindow('theUniqueLabel', {
			url: 'index.html'
		});
		// since the webview window is created asynchronously,
		// Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
		webview.once('tauri://created', function () {
			// webview window successfully created
		});
		webview.once('tauri://error', function (e) {
			// an error occurred during webview window creation
		});
	});

	setInterval(() => {
		amogus.style.scale = `${(Math.random() * 2 - 1) * 20 + 100}%`;
	}, 0);
</script>

<div class="h-screen w-screen border-red border-solid border-2">
	<div class="grid h-full w-full items-center justify-items-center">
		<div>
			<h1>Welcome to SvelteKit</h1>
			<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
			<img class="opacity-5" bind:this={amogus} src="amogus.png" alt="amogus" />
		</div>
	</div>
</div>
