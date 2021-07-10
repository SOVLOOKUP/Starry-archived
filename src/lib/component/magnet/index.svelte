<script lang="ts">
	import ContextMenu from './component/menu.svelte';
	import type { Menu, MenuItem } from './component/menu.svelte';
	import { once } from 'svelte/internal';

	export let imgURL = 'back.jpg';
	export let drag = true;
	export let items: MenuItem[] = [];

	let magnet: HTMLDivElement;
	let menuConfig: Menu = {
		menuVisible: false,
		X: 0,
		Y: 0,
		items: [
			{
				name: drag ? '锁定' : '解锁',
				onClick: () => (drag = !drag)
			},
			...items
		]
	};

	const menu = (e: MouseEvent) => {
		e.preventDefault();

		// 隐藏菜单挂钩，运行一次销毁
		if (e.button === 2 && !menuConfig.menuVisible) {
			magnet.addEventListener(
				'mousedown',
				once((e: MouseEvent) => {
					if (e.button !== 2) menuConfig.menuVisible = false;
				})
			);
		}

		menuConfig.items[0].name = drag ? '锁定' : '解锁';
		menuConfig.menuVisible = !menuConfig.menuVisible;
		menuConfig.X = e.clientX;
		menuConfig.Y = e.clientY;
	};

	const click = (e: MouseEvent) => {
		console.log(e);
	};

	// 可被tauri窗口拖拽
	$: drag;
</script>

<div class="magnet" bind:this={magnet} on:contextmenu={menu}>
	<div
		class="background"
		style="background-image: url({imgURL});"
		data-tauri-drag-region={drag ? '' : 'undragable'}
	/>
</div>
<ContextMenu {menuConfig} />

<style>
	.magnet {
		height: 100%;
		width: 100%;
	}

	.background {
		user-select: none;
		position: fixed;
		height: 100%;
		width: 100%;
		z-index: -999;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		border-radius: 20px;
		background-size: cover;
	}
</style>
