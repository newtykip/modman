<script lang="ts">
	import type { Loader } from "$lib/loader";
	const size = 50;

	export let loader: string;
	export let group: Loader;
</script>

<label>
	<input type="radio" class="absolute opacity-0 w-0 h-0" value={loader} bind:group />
	<img
		src={`/images/loaders/${loader.toLowerCase()}.png`}
		alt={loader}
		width={size}
		height={size}
		class={`pointer-events-auto cursor-pointer ${
			group ? (group.toString() === loader ? "border-2 border-red-500" : "") : ""
		}`}
	/>
</label>
