<script lang="ts">
	import type { Profile } from "$lib/bindings";
	export let profile: Profile;
</script>

<div class="relative hover:cursor-pointer shadow-lg w-[210px] h-[120px]">
	<!-- todo: custom profile image -->
	<img src={`${profile.loader}.png`} class="h-full w-full object-cover" alt="" />
	<div
		class="w-full h-full bg-black/75 backdrop-brightness-20 absolute top-0 flex flex-col items-center justify-center text-white rounded-sm profile"
	>
		<p class="absolute top-1 right-2 text-xs text-gray-300">{profile.minecraft_version}</p>
		<p class="text-2xl font-bold">{profile.name}</p>
		<p class="text-lg">by {profile.author}</p>
	</div>
	<div
		class="w-full h-full absolute top-0 flex items-center justify-center text-white opacity-0 hover:opacity-100 ease-out bg-black/50 backdrop-blur-sm duration-[0.15s] rounded-sm"
	>
		<p class="text-3xl font-bold">Edit</p>
	</div>
</div>
