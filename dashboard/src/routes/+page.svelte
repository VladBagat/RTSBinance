<script lang="ts">
	import { onMount } from 'svelte';

	let events: string[] = [];

	onMount(() => {
		const es = new EventSource('/kafka-sink/messages');

		es.onmessage = (e) => {
			events = [...events, JSON.parse(e.data)];
		};

		es.onerror = () => {
			es.close();
		};
	});
</script>

{#each events as event}
<p>{event}</p>
{/each}
