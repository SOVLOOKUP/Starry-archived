<script lang="ts">
	import ContextMenu from './component/menu.svelte';
	import type { Menu, MenuItem } from './component/menu.svelte';
	import { once } from 'svelte/internal';
	import { appWindow, LogicalSize } from '$lib/tauri/window';

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
				name: drag ? 'Lock' : 'Unlock',
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

	// 可被tauri窗口拖拽
	$: (async () => await appWindow.setResizable(drag))();
</script>

<div class="magnet" bind:this={magnet} on:contextmenu={menu}>
	<div class="background" style="background-image: url({imgURL});" />
	{#if drag}
		<div class="dragRegion" data-tauri-drag-region={drag ? '' : 'undragable'} />
	{/if}
	<div
		class="content"
		on:contextmenu={(e) => {
			e.preventDefault();
		}}
	>
		<button>磁贴内容块</button>
	</div>
</div>

<ContextMenu {menuConfig} />

<style>
	.content {
		position: absolute;
		display: flex;
	}

	.magnet {
		position: absolute;
		height: 100%;
		width: 100%;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		border-radius: 20px;
	}

	.dragRegion {
		position: absolute;
		border-radius: 20px;
		background-color: rgb(255, 255, 255, 0);
		top: 5px;
		left: 5px;
		right: 5px;
		bottom: 5px;
	}

	.background {
		position: fixed;
		user-select: none;
		top: 0;
		z-index: -999;
		height: 100%;
		width: 100%;
		background-size: cover;
		border-radius: 20px;
	}
</style>
