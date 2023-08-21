<script lang="ts">
	import Loading from "$components/Loading.svelte";
	import Meta from "$components/Meta.svelte";
	import { getProfile } from "$lib/bindings";

	export let data: ReturnType<typeof import("$routes/view/[slug]/+page").load>;
</script>

<!-- todo: complete this component -->
{#await getProfile(data.slug)}
	<Loading />
{:then profile}
	<Meta title={profile.name} back={true} />
	{profile.author}
{/await}
