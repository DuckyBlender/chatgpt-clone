<script lang="ts">
	import { onMount } from 'svelte';
	import { tweened } from 'svelte/motion';
	let username = 'Human';

	let message = '';
	let messages: { name: string; message: string }[] = [];
	var thinking = false;
	const timeout = 3;
	// Global cooldown bool which can be changed from a function
	let cooldown = false;
	// TODO: Add a cooldown to the button

	// Focus the textview
	onMount(() => {
		let input = document.getElementById('messageInput');
		if (input !== null) {
			input.focus();
		}
	});

	function saveMessages() {
		let text = '';
		messages.forEach((msg) => {
			text += msg.name + ': ' + msg.message + '\n';
		});
		let element = document.createElement('a');
		element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
		element.setAttribute('download', 'chat.txt');

		element.style.display = 'none';
		document.body.appendChild(element);

		element.click();

		document.body.removeChild(element);
		// I have actually no idea how this works, copilot did it for me lol
	}

	// add messages to the array
	async function addMessage(name: string, message: string) {
		if (cooldown) return;
		// buttonCooldown();
		messages = [...messages, { name, message }];
		if (name === 'ChatGPT') return;

		// send the message to the server
		let requestBody = {
			model: 'gpt-3.5-turbo',
			messages: messages.map((msg) => {
				return {
					role: msg.name === 'ChatGPT' ? 'assistant' : 'user',
					content: msg.message
				};
			})
		};

		thinking = true;
		let res = await fetch('https://api.openai.com/v1/chat/completions', {
			method: 'POST',
			headers: {
				// to hide this from the public, I have added this to the .env file
				Authorization: 'Bearer ' + import.meta.env.VITE_OPENAI_API_KEY,
				// to set this up in svelte using vite
				'Content-Type': 'application/json'
			},

			body: JSON.stringify(requestBody)
		});
		// get the response from the server
		if (res.ok) {
			let response = (await res.json()).choices[0].message.content;
			console.log(response);
			addMessage('ChatGPT', response);
		} else {
			addMessage('ChatGPT', 'Something went wrong, please try again later.');
		}
		thinking = false;
	}

	addMessage('ChatGPT', 'Ask me anything!');
</script>

<!-- If the user is logged in, show the chat -->
{#each messages as msg}
	{#if msg.name === username}
		<!-- Human -->
		<div class="bg-blue-700 text-white rounded-lg p-2 my-2 whitespace-pre-line shadow-md">
			{msg.message}
			<script>
				console.log(msg.message);
			</script>
		</div>
	{:else}
		<!-- ChatGPT -->
		<div class="bg-gray-700 text-white rounded-lg p-2 my-2 whitespace-pre-line shadow-md">
			{msg.message}
		</div>
	{/if}
{/each}
{#if thinking}
	<div class="bg-gray-700 text-gray-500 rounded-lg p-2 my-2 whitespace-pre-line shadow-md">
		Thinking...
	</div>
{/if}
<!-- To fix shift+enter functionality in the above input, we need to use a textarea -->
<textarea
	bind:value={message}
	on:keydown={(e) => {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			addMessage(username, message);
			message = '';
		}
	}}
	placeholder="Your message"
	class="rounded-md p-2 w-full shadow-md"
	rows="3"
	id="messageInput"
/>

<!-- Small reminder that the user can adjust the height by dragging it -->
<p class="text-xs text-gray-500">
	You can adjust the height of the input box by dragging the bottom of it
</p>

<div class="flex flex-row space-x-2">
	{#if cooldown === false}
		<button
			on:click={() => addMessage(username, message)}
			class="bg-blue-500 text-white rounded-md p-2 shadow-md flex-grow"
		>
			<!-- On click reset the message -->
			Reply
		</button>
	{:else}
		<button disabled class="bg-blue-500 text-white rounded-md p-2 shadow-md flex-grow">
			<!-- On click reset the message -->
			Reply
		</button>
	{/if}

	<button
		on:click={() => {
			messages = [];
			addMessage('ChatGPT', 'Ask me anything!');
			message = '';
			// Also reset the textview
			let input = document.getElementById('messageInput');
			if (input !== null) {
				// input.value doesnt work
				input.innerText = '';
				input.focus();
			}
		}}
		class="bg-blue-500 text-white rounded-md p-2 shadow-md"
	>
		<!-- On click reset the message -->
		Reset
	</button>
	<button on:click={saveMessages} class="bg-blue-500 text-white rounded-md p-2 shadow-md">
		<!-- On click get all of the messages and save them to a txt file -->
		Save
	</button>
</div>
