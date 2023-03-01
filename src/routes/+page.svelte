<script lang="ts">
	let name = '';
	let loggedIn = false;

	let message = '';
	let messages: { name: string; message: string }[] = [];
	let replying = false;

	// add messages to the array
	function addMessage(name: string, message: string) {
		messages = [...messages, { name, message }];
		if (name !== 'ChatGPT') {
			// send the message to the server
			replying = true;
			let requestBody = {
				model: 'gpt-3.5-turbo',
				messages: [
					{
						role: 'user',
						text: message
					}
				]
			};
			fetch('https://api.openai.com/v1/chat/completions', {
				method: 'POST',
				headers: {
					Authorization: 'Bearer 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',
					'Content-Type': 'application/json'
				},
				// body: JSON.stringify({ requestBody })
				// to fix the model is not found

				body: JSON.stringify(requestBody)
			})
				.then((res) => res.json())
				.then((res) => {
					// get the response from the server
					console.log(res);
					let response = res.choices[0].message.content;
					addMessage('ChatGPT', response);
				});
			replying = false;
		}
	}

	addMessage('ChatGPT', 'Welcome to ChatGPT!');
	// If the user inputs a name, store it in localStorage
	// so that it can be used in the next session
	function startChatting() {
		if (name !== '') {
			localStorage.setItem('name', name);
			loggedIn = true;
		} else {
			alert('Please enter a name');
		}
	}
</script>

<h1>Welcome to ChatGPT!</h1>
<p>ChatGPT is a chatbot that uses GPT-3.5 to generate responses to your messages.</p>
<p>It's still in development, so please be patient with it.</p>

<p>To start, enter your name and click "Start Chatting".</p>
<input type="text" bind:value={name} placeholder="Your name" />
<button on:click={startChatting}>Start Chatting</button>

{#if loggedIn}
	<br /><br />
	<!-- If the user is logged in, show the chat -->
	{#each messages as msg}
		{#if msg.name === name}
			<!-- using tailwind css -->
			<div class="bg-blue-500 text-white rounded-lg p-2">
				{name}: {msg.message}
			</div>
		{:else}
			<div class="bg-gray-500 text-white rounded-lg p-2">
				ChatGPT: {msg.message}
			</div>
		{/if}
	{/each}
	<input type="text" bind:value={message} placeholder="Your message" />
	{#if replying}
		<button
			on:click={() => addMessage(name, message)}
			disabled
			class="bg-gray-500 text-white rounded-lg p-2"
		>
			ChatGPT is replying...
		</button>
	{:else}
		<button
			on:click={() => addMessage(name, message)}
			class="bg-blue-500 text-white rounded-lg p-2"
		>
			<!-- On click reset the message -->
			Reply
		</button>
	{/if}
{/if}
