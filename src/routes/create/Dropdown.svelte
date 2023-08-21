<script lang="ts">
	export let label: string;
	export let items: string[];
</script>

<div class="flex items-center mb-6">
	<div class="w-1/3">
		<label class="block font-bold text-right mb-1 pr-4" for={label}>{label}</label>
	</div>

	<div class="w-2/3">
		<select
			class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500"
			id={label}
		>
			{#each items as item}
				<option value={item}>{item}</option>
			{/each}
		</select>
	</div>
</div>
