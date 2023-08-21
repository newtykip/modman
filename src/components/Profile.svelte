<script lang="ts">
	import { goto } from "$app/navigation";
	import { createSlug, type Profile } from "$lib/bindings";

	export let profile: Profile;
	export let number: number;

	const view = async () => {
		goto(`/view/${await createSlug(profile.name)}`);
	};
</script>

<div class="relative hover:cursor-pointer shadow-lg w-[210px] h-[120px]">
	<!-- todo: custom profile image -->
	<img src={`${profile.loader}.png`} class="h-full w-full object-cover" alt="" />
	<div
		class="w-full h-full bg-black/75 backdrop-brightness-20 absolute top-0 flex flex-col items-center justify-center text-white rounded-sm profile"
	>
		<p class="absolute top-1 right-2 text-xs text-gray-300">{profile.minecraftVersion}</p>
		<p class="text-2xl font-bold">{profile.name}</p>
		<p class="text-lg">by {profile.author}</p>
	</div>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="w-full h-full absolute top-0 flex items-center justify-center text-white opacity-0 hover:opacity-100 ease-out bg-black/50 backdrop-blur-sm duration-[0.15s] rounded-sm"
		on:click={view}
		role="button"
		tabindex={number}
	>
		<p class="text-3xl font-bold">Edit</p>
	</div>
</div>
