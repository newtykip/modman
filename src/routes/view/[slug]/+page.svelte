<script lang="ts">
	import Loading from "$components/Loading.svelte";
	import Meta from "$components/Meta.svelte";
	import { getProfile } from "$lib/bindings";
	import { loaderToString } from "$lib/loader";

	export let data: ReturnType<typeof import("$routes/view/[slug]/+page").load>;
</script>

<!-- todo: complete this component -->
{#await getProfile(data.slug)}
	<Loading />
{:then profile}
	<Meta title={profile.name} />

	<div class="grid grid-cols-2 m-12">
		<div>
			<h1 class="text-5xl font-bold">
				{profile.name}
			</h1>
			<h2 class="text-2xl">by {profile.author}, v{profile.version}</h2>
		</div>
		<div class="text-xl text-right">
			<p>
				<img src={`/images/grass.webp`} alt="minecraft" class="inline" height={25} width={25} />
				{profile.minecraft_version}
			</p>
			<p>
				<img
					src={`/images/loaders/${loaderToString(profile.loader)}.png`}
					alt={loaderToString(profile.loader)}
					class="inline"
					height={25}
					width={25}
				/>
				{profile.loader_version}
			</p>
		</div>
	</div>
{/await}
