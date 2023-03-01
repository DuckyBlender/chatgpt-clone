<script lang="ts">
	let name = '';
	let loggedIn = false;

	let message = '';
	let messages: { name: string; message: string }[] = [];
	var replying = false;

	// add messages to the array
	function addMessage(name: string, message: string) {
		messages = [...messages, { name, message }];
		if (name !== 'ChatGPT') {
			// send the message to the server
			replying = true;
			let requestBody = {
				model: 'gpt-3.5-turbo',
				messages: messages.map((msg) => {
					return {
						role: msg.name === 'ChatGPT' ? 'assistant' : 'user',
						content: msg.message
					};
				})
			};
			fetch('https://api.openai.com/v1/chat/completions', {
				method: 'POST',
				headers: {
					Authorization: 'Bearer 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',
					'Content-Type': 'application/json'
				},

				body: JSON.stringify(requestBody)
			})
				.then((res) => res.json())
				.then((res) => {
					// get the response from the server
					console.log(res);
					let response = res.choices[0].message.content;
					addMessage('ChatGPT', response);
				});
			console.log('replying: ' + replying);
			replying = false;
		}
	}

	addMessage('ChatGPT', 'Ask me anything!');
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

{#if loggedIn === false}
	<div class="font-lexend text-center border-b-2 text-white mb-4">
		<h1>Welcome to ChatGPT!</h1>
		<p>ChatGPT is a chatbot that uses GPT-3.5 to generate responses to your messages.</p>
		<p>It's still in development, so please be patient with it.</p>

		<p>To start, enter your name and click "Start Chatting".</p>
	</div>
	<div class="font-lexend text-center text-black">
		<input type="text" bind:value={name} placeholder="Your name" class="rounded-md p-2" />
		<button on:click={startChatting} class="text-white bg-blue-500 rounded-md p-2">
			<!-- On click start chatting -->
			Start Chatting
		</button>
	</div>
{:else}
	<!-- If the user is logged in, show the chat -->
	{#each messages as msg}
		{#if msg.name === name}
			<!-- using tailwind css -->
			<div class="bg-blue-700 text-white rounded-lg p-2 my-2">
				{name}: {msg.message}
			</div>
		{:else}
			<div class="bg-gray-700 text-white rounded-lg p-2 my-2">
				ChatGPT: {msg.message}
			</div>
		{/if}
	{/each}
	<input type="text" bind:value={message} placeholder="Your message" class="rounded-md p-2" />
	{#if replying}
		<button
			on:click={() => addMessage(name, message)}
			disabled
			class="bg-gray-500 text-white rounded-md p-2"
		>
			Replying...
		</button>
	{:else}
		<button
			on:click={() => addMessage(name, message)}
			class="bg-blue-500 text-white rounded-md p-2"
		>
			<!-- On click reset the message -->
			Reply
		</button>
	{/if}
	<button
		on:click={() => {
			messages = [];
			addMessage('ChatGPT', 'Ask me anything!');
		}}
		class="bg-blue-500 text-white rounded-md p-2"
	>
		<!-- On click reset the message -->
		Reset
	</button>
	<button
		on:click={() => {
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
		}}
		class="bg-blue-500 text-white rounded-md p-2"
	>
		<!-- On click get all of the messages and save them to a txt file -->
		Save
	</button>
{/if}
