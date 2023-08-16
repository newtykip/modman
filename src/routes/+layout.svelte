<script lang="ts">
	import "../app.postcss";
	import { appWindow } from "@tauri-apps/api/window";
</script>

<div
	data-tauri-drag-region
	class="bg-titlebar sticky top-0 flex flex-row items-center justify-end h-[45px]"
>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="text-sm text-white w-[45px] h-full flex items-center justify-center hover:bg-titlebar-highlight hover:cursor-pointer"
		on:click={() => appWindow.minimize()}
		role="button"
		tabindex={0}
	>
		<svg
			aria-hidden="true"
			focusable="false"
			data-prefix="fas"
			data-icon="window-minimize"
			class="w-3"
			role="img"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 512 512"
			><path
				fill="currentColor"
				d="M464 352H48c-26.5 0-48 21.5-48 48v32c0 26.5 21.5 48 48 48h416c26.5 0 48-21.5 48-48v-32c0-26.5-21.5-48-48-48z"
			/></svg
		>
	</div>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="text-sm text-white w-[45px] h-full flex items-center justify-center hover:bg-titlebar-highlight hover:cursor-pointer"
		on:click={() => appWindow.toggleMaximize()}
		role="button"
		tabindex={1}
	>
		<svg
			aria-hidden="true"
			focusable="false"
			data-prefix="fas"
			data-icon="window-maximize"
			class="w-3"
			role="img"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 512 512"
			><path
				fill="currentColor"
				d="M464 32H48C21.5 32 0 53.5 0 80v352c0 26.5 21.5 48 48 48h416c26.5 0 48-21.5 48-48V80c0-26.5-21.5-48-48-48zm-16 160H64v-84c0-6.6 5.4-12 12-12h360c6.6 0 12 5.4 12 12v84z"
			/></svg
		>
	</div>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="text-white w-[45px] h-full flex items-center justify-center hover:bg-[#d64441] hover:cursor-pointer"
		on:click={() => appWindow.close()}
		role="button"
		tabindex={2}
	>
		<svg
			aria-hidden="true"
			focusable="false"
			data-prefix="fas"
			data-icon="times"
			class="w-3"
			role="img"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 352 512"
			><path
				fill="currentColor"
				d="M242.72 256l100.07-100.07c12.28-12.28 12.28-32.19 0-44.48l-22.24-22.24c-12.28-12.28-32.19-12.28-44.48 0L176 189.28 75.93 89.21c-12.28-12.28-32.19-12.28-44.48 0L9.21 111.45c-12.28 12.28-12.28 32.19 0 44.48L109.28 256 9.21 356.07c-12.28 12.28-12.28 32.19 0 44.48l22.24 22.24c12.28 12.28 32.2 12.28 44.48 0L176 322.72l100.07 100.07c12.28 12.28 32.2 12.28 44.48 0l22.24-22.24c12.28-12.28 12.28-32.19 0-44.48L242.72 256z"
			/></svg
		>
	</div>
</div>

<slot />
