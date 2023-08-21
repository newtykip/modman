<script lang="ts">
	import Loading from "$components/Loading.svelte";
	import Meta from "$components/Meta.svelte";
	import { getProfile } from "$lib/bindings";
	import title from "title";

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
				<img
					src={`/images/grass.webp`}
					alt={profile.loader}
					class="inline"
					height={25}
					width={25}
				/>
				{profile.minecraftVersion}
			</p>
			<p>
				<img
					src={`/images/loaders/${profile.loader}.png`}
					alt={profile.loader}
					class="inline"
					height={25}
					width={25}
				/>
				{profile.loaderVersion}
			</p>
		</div>
	</div>
{/await}
