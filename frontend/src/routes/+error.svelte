<!-- Simple error page -->
<script lang="ts">
	import { onMount } from 'svelte';

	onMount(() => {
		// Check if the user is logged in
		if (localStorage.getItem('token') || localStorage.getItem('userid')) {
			// If the user is logged in, redirect to the chat page
			window.location.href = '/chat';
		} else {
			window.location.href = '/';
		}
	});
</script>
