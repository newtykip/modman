<script lang="ts">
	import { loadProfiles, saveProfile } from "$lib/bindings";
	import Profile from "./Profile.svelte";
	import { onMount } from "svelte";
	import Meta from "$components/Meta.svelte";
	import Loading from "$components/Loading.svelte";
	import { goto } from "$app/navigation";

	let colCount: number;

	onMount(() => {
		colCount = Math.floor(window.innerWidth / 250);
		window.addEventListener("resize", () => {
			colCount = Math.floor(window.innerWidth / 250);
		});
	});
</script>

<Meta title="View Profiles" back={false} />
<h1 class="text-5xl font-bold m-12">Select a profile</h1>

<div class="mx-12">
	{#await loadProfiles()}
		<Loading />
	{:then profiles}
		<div class={`grid grid-cols-${colCount} gap-10`}>
			<div class="relative hover:cursor-pointer shadow-lg w-[210px] h-[120px]">
				<img src="/images/blahaj.jpg" class="h-full w-full object-cover" alt="" />
				<div
					class="w-full h-full bg-black/75 backdrop-brightness-20 absolute top-0 flex flex-col items-center justify-center rounded-sm profile"
				>
					<p class="text-2xl font-bold">New Profile</p>
				</div>
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div
					class="w-full h-full absolute top-0 flex items-center justify-center opacity-0 hover:opacity-100 ease-out bg-black/50 backdrop-blur-sm duration-[0.15s] rounded-sm"
					on:click={() => goto("/create")}
					role="button"
					tabindex={0}
				>
					<p class="text-3xl font-bold">Create</p>
				</div>
			</div>

			{#each profiles as profile, i}
				<Profile {profile} number={i + 1} />
			{/each}
		</div>
	{/await}
</div>
