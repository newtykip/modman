<script lang="ts">
	import { loadProfiles } from "$lib/bindings";
	import Profile from "$components/Profile.svelte";
	import { onMount } from "svelte";
	import Meta from "$components/Meta.svelte";
	import Loading from "$components/Loading.svelte";

	let colCount: number;

	onMount(() => {
		colCount = Math.floor(window.innerWidth / 250);
		window.addEventListener("resize", () => {
			colCount = Math.floor(window.innerWidth / 250);
		});
	});
</script>

<Meta title="View Profiles" />

<h1 class="text-center text-5xl font-black my-12 font-warming">Select a profile</h1>

<div class="mx-12">
	{#await loadProfiles()}
		<Loading />
	{:then profiles}
		<div class={`grid grid-cols-${colCount} gap-10`}>
			<div class="relative hover:cursor-pointer shadow-lg w-[210px] h-[120px]">
				<img src="blahaj.jpg" class="h-full w-full object-cover" alt="" />
				<div
					class="w-full h-full bg-black/75 backdrop-brightness-20 absolute top-0 flex flex-col items-center justify-center rounded-sm profile"
				>
					<p class="text-2xl font-bold">New Profile</p>
				</div>
				<div
					class="w-full h-full absolute top-0 flex items-center justify-center opacity-0 hover:opacity-100 ease-out bg-black/50 backdrop-blur-sm duration-[0.15s] rounded-sm"
				>
					<p class="text-3xl font-bold">Create</p>
				</div>
			</div>

			{#each profiles as profile, i}
				<!-- todo: scale on hover -->
				<Profile {profile} number={i} />
			{/each}
		</div>
	{/await}
</div>
