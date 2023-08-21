<!-- todo: create loader indexes before this -->

<script lang="ts">
	import Meta from "$components/Meta.svelte";
	import RadioLoader from "./Loader.svelte";
	import TextInput from "./TextInput.svelte";
	import type { Loader } from "$lib/bindings";
	import Loading from "$components/Loading.svelte";
	import Dropdown from "./Dropdown.svelte";

	// fetch minecraft versions
	let minecraftVersions = fetch("https://piston-meta.mojang.com/mc/game/version_manifest.json")
		.then((res) => res.json())
		.then((res) =>
			res["versions"]
				.filter((version: any) => version["type"] === "release")
				.map((version: any) => version["id"]),
		);

	let name: string;
	let author: string;
	let minecraftVersion: string;
	let loader: Loader;

	$: valid = name && author && loader;
</script>

<Meta title="Create Profile" />

<h1 class="text-5xl font-bold m-12">Create a profile</h1>

{#await minecraftVersions}
	<Loading />
{:then minecraftVersions}
	<form class="max-w-[550px]">
		<TextInput label="Name" bind:value={name} max={30} />
		<TextInput label="Author" bind:value={author} max={30} />

		<div class="flex items-center mb-6">
			<div class="w-1/3">
				<p class="block font-bold text-right mb-1 pr-4">Loader</p>
			</div>
			<div class="w-2/3 flex gap-5">
				<RadioLoader loader="forge" bind:group={loader} />
				<RadioLoader loader="fabric" bind:group={loader} />
				<RadioLoader loader="quilt" bind:group={loader} />
			</div>
		</div>

		{#if loader}
			<Dropdown label="Minecraft Version" items={minecraftVersions} />
		{/if}

		<div class="flex">
			<div class="w-1/3" />
			<div class="w-2/3">
				<button
					class={`shadow bg-${valid ? "purple" : "red"}-500 hover:bg-${
						valid ? "purple" : "red"
					}-400 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded`}
					type="button"
					disabled={!name}
				>
					{#if name}
						Create {name}
					{:else}
						Please enter a name
					{/if}
				</button>
			</div>
		</div>
	</form>
{/await}
