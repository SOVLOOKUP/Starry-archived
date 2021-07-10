<script context="module" lang="ts">
	export interface Menu {
		X: number;
		Y: number;
		menuVisible: boolean;
		items: MenuItem[];
	}

	export type MenuItem = {
		name: string;
		onClick: (event: MouseEvent & { currentTarget: EventTarget & HTMLSpanElement }) => void;
	};
</script>

<script lang="ts">
	export let menuConfig: Menu;

	$: menuConfig;
</script>

{#if menuConfig.menuVisible}
	<!-- todo:菜单美化 -->
	<div style="top: {menuConfig.Y + 1}px; left: {menuConfig.X + 1}px;" class="contextMenu">
		{#each menuConfig.items as item}
			<span
				on:click={(e) => {
					menuConfig.menuVisible = !menuConfig.menuVisible;
					item.onClick(e);
				}}>{item.name}</span
			>
		{/each}
	</div>
{/if}

<style>
	.contextMenu {
		background-color: rgb(255, 255, 255);
		user-select: none;
		position: fixed;
		display: flex;
		flex-flow: column;
	}
</style>
