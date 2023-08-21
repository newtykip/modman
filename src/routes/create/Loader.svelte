<script lang="ts">
	import type { Loader } from "$lib/bindings";
	const size = 50;

	export let loader: Loader;
	export let group: Loader;
</script>

<label>
	<input type="radio" class="absolute opacity-0 w-0 h-0" value={loader} bind:group />
	<img
		src={`/images/loaders/${loader}.png`}
		alt={loader}
		width={size}
		height={size}
		class={`pointer-events-auto cursor-pointer ${
			group === loader ? "border-2 border-red-500" : ""
		}`}
	/>
</label>
