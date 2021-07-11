<script lang="ts">
	import SearchBar from './_component/searchBar.svelte';

	const disableContextMenu = (e: MouseEvent) => {
		if (!!window.__TAURI__) e.preventDefault();
	};
</script>

<svelte:head>
	<title>Starry</title>
	<link rel="icon" href="logo.svg" />
</svelte:head>

<main on:contextmenu={disableContextMenu} class="main">
	<!-- todo: auto find the best size -->
	<!-- todo: 国际化 -->
	<div class="searchBar"><SearchBar /></div>

	<div class="content"><slot /></div>
</main>

<style>
	@font-face {
		font-family: HarmonyOS;
		src: 'HarmonyOS.ttf';
	}

	.main {
		font-family: 'HarmonyOS';
	}

	.searchBar {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
	}

	.content {
		position: absolute;
		top: 3.2em;
		left: 0;
		right: 0;
		bottom: 0;
	}
</style>
