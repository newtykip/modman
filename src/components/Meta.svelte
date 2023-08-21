<script lang="ts">
	import * as stores from "$stores";

	export let title: string;
	export let back: boolean = false;

	stores.title.set(title);
	stores.back.set(back);
</script>
