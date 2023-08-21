<script lang="ts">
	import "../app.postcss";
	import { appWindow } from "@tauri-apps/api/window";
	import { title, back } from "$stores";
	import { onMount } from "svelte";
	import { ArrowLeftIcon, MaximizeIcon, MinimizeIcon, XIcon } from "svelte-feather-icons";

	onMount(() => {
		// disable reloading
		window.addEventListener("keydown", (e) => {
			if (e.code === "F5" || (e.ctrlKey && e.code === "KeyR")) {
				e.preventDefault();
			}
		});
	});
</script>

<div
	data-tauri-drag-region
	class="bg-titlebar sticky top-0 h-[45px] w-full grid grid-cols-3 items-center"
>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	{#if $back}
		<div
			class="text-sm text-white w-[45px] h-full flex items-center justify-center hover:bg-titlebar-highlight hover:cursor-pointer"
			on:click={() => history.back()}
			role="button"
			tabindex={0}
		>
			<ArrowLeftIcon />
		</div>
	{:else}
		<div />
	{/if}
	<div class="text-center pointer-events-none">{$title}</div>
	<div class="flex items-center h-full absolute right-0">
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<div
			class="text-sm text-white w-[45px] h-full flex items-center justify-center hover:bg-titlebar-highlight hover:cursor-pointer"
			on:click={() => appWindow.minimize()}
			role="button"
			tabindex={1}
		>
			<MinimizeIcon />
		</div>
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<div
			class="text-sm text-white w-[45px] h-full flex items-center justify-center hover:bg-titlebar-highlight hover:cursor-pointer"
			on:click={() => appWindow.toggleMaximize()}
			role="button"
			tabindex={2}
		>
			<MaximizeIcon />
		</div>
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<div
			class="text-white w-[45px] h-full flex items-center justify-center hover:bg-[#d64441] hover:cursor-pointer"
			on:click={() => appWindow.close()}
			role="button"
			tabindex={3}
		>
			<XIcon />
		</div>
	</div>
</div>

<slot />
