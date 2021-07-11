<script lang="ts">
	export let placeHolder = 'Hi, Starry✨';
	export let inputValue = '';
	export let height = 3.2;

	const clickLogo = (e: MouseEvent) => {
		console.log(e, inputValue);
	};

	$: [placeHolder, inputValue, height];
</script>

<div class="searchBar" style="height: {height}em;">
	<div class="searchArea">
		<input class="searchInput" type="text" placeholder={placeHolder} bind:value={inputValue} />
	</div>
	<div on:click={clickLogo} class="logo">
		<img class="logoSVG" data-tauri-drag-region alt="logo" src="logo.svg" />
	</div>
</div>

<style>
	/* todo:美化动态效果
	todo:主界面和副界面模式 */
	.searchBar {
		display: flex;
		height: 3.2em;
		justify-content: center;
		align-items: center;

		background: rgba(248, 248, 248, 0.75);
		box-shadow: inset 4px 4px 15px 1px rgba(2, 42, 88, 0.25);
		border-radius: 20px;
	}

	.searchArea {
		position: absolute;
		display: flex;
		justify-content: center;
		align-items: center;
		right: 4em;
		left: 1em;
		height: 80%;
	}

	.searchInput {
		position: absolute;
		height: 100%;
		width: 100%;

		background: rgba(255, 255, 255, 0);
		outline-style: none;
		border: 0px;
		font-size: 2em;
	}

	.logoSVG {
		height: 100%;
	}

	.logo {
		position: fixed;
		display: flex;
		justify-content: center;
		align-items: center;

		height: 2.5em;
		right: 6px;
		border-radius: 1.25em;

		background: #022a58;
		box-shadow: 0px 4px 4px rgba(2, 42, 88, 0.5);
	}
</style>
