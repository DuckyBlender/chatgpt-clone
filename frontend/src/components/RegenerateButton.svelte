<script lang="ts">
	export let messages: any;
	export let thinking: any;
	export let cooldown: any;
	export let shakeButton: any;
	export let addMessage: any;
</script>

<div class="float-right inline-block h-6 w-6 align-top text-gray-500 invert filter">
	<button
		on:click={() => {
			if (thinking || cooldown) {
				shakeButton();
				return;
			}
			let lastMessage = messages[messages.length - 2];
			messages.pop();
			messages.pop();
			addMessage(lastMessage.name, lastMessage.message);
		}}
	>
		<img src="/regen.svg" class="inline-block h-6 w-6 px-1" alt="Regenerate" />
	</button>
</div>
