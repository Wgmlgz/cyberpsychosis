<script lang="ts">
	import 'virtual:uno.css';
	import 'carbon-components-svelte/css/white.css';

	import {
		Header,
		HeaderNav,
		HeaderNavItem,
		SkipToContent,
		HeaderPanelDivider,
		HeaderPanelLinks,
		HeaderAction,
		HeaderUtilities,
		HeaderGlobalAction
	} from 'carbon-components-svelte';
	import '$lib/index.css';

	import { onMount } from 'svelte';
	import { appWindow, LogicalSize } from '@tauri-apps/api/window';
	// onMount(async () => {
	// 	await appWindow.setSize(new LogicalSize(0, 0));
	// 	await appWindow.setSize(new LogicalSize(1000, 1000));
	// });
</script>

<slot />
