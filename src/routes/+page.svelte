<script lang="ts">
	import { loadProfiles } from "$lib/bindings";
	import Profile from "$components/Profile.svelte";
	import { onMount } from "svelte";

	let width: number;

	onMount(() => {
		width = window.innerWidth;
		window.addEventListener("resize", () => (width = window.innerWidth));
	});
</script>

<h1 class="text-center text-white text-6xl font-black mt-12 mb-10 font-warming">Profiles</h1>

<div class="mx-12">
	{#await loadProfiles()}
		Loading...
	{:then profiles}
		<div class={`grid grid-cols-${Math.floor(width / 250)} gap-10`}>
			<div class="relative hover:cursor-pointer shadow-lg w-[210px] h-[120px]">
				<img src="blahaj.jpg" class="h-full w-full object-cover" alt="" />
				<div
					class="w-full h-full bg-black/75 backdrop-brightness-20 absolute top-0 flex flex-col items-center justify-center text-white rounded-sm profile"
				>
					<p class="text-2xl font-bold">New Profile</p>
				</div>
				<div
					class="w-full h-full absolute top-0 flex items-center justify-center text-white opacity-0 hover:opacity-100 ease-out bg-black/50 backdrop-blur-sm duration-[0.15s] rounded-sm"
				>
					<p class="text-3xl font-bold">Create</p>
				</div>
			</div>

			{#each profiles as profile}
				<!-- todo: scale on hover -->
				<Profile {profile} />
			{/each}
			{#each profiles as profile}
				<!-- todo: scale on hover -->
				<Profile {profile} />
			{/each}
			{#each profiles as profile}
				<!-- todo: scale on hover -->
				<Profile {profile} />
			{/each}
			{#each profiles as profile}
				<!-- todo: scale on hover -->
				<Profile {profile} />
			{/each}
			{#each profiles as profile}
				<!-- todo: scale on hover -->
				<Profile {profile} />
			{/each}
			{#each profiles as profile}
				<!-- todo: scale on hover -->
				<Profile {profile} />
			{/each}
			{#each profiles as profile}
				<!-- todo: scale on hover -->
				<Profile {profile} />
			{/each}
		</div>
	{/await}
</div>
