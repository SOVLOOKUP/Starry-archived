<script>
	import SearchBar from './_component/searchBar.svelte';
</script>

<svelte:head>
	<title>Starry</title>
	<link rel="icon" href="logo.svg" />
</svelte:head>

<main class="main">
	<div class="searchBar"><SearchBar /></div>

	<div class="content"><slot /></div>

	<img class="background" alt="back" src="back.jpg" />
</main>

<style>
	@font-face {
		font-family: HarmonyOS;
		src: 'HarmonyOS.ttf';
	}

	.main {
		font-family: 'HarmonyOS';
	}

	.background {
		position: fixed;
		height: 100%;
		z-index: -999;
		top: 0;
		left: 0;
		object-fit: cover;
		width: 100%;
		border-radius: 20px;
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
	}
</style>
