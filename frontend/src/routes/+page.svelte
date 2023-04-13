<!-- It will show the login form and the anonymous "input token" form -->
<script lang="ts">
	import { onMount } from 'svelte';

	let token = '';
	let username = '';
	let password = '';

	onMount(() => {
		// Check if the user is logged in
		if (localStorage.getItem('token') || localStorage.getItem('session')) {
			// If the user is logged in, redirect to the chat page
			window.location.href = '/chat';
		}
	});

	async function loginPassword() {
		console.log('Logging in...');
		// Get the username and password and register token from the form
		let username = (document.getElementById('username') as HTMLInputElement).value;
		let password = (document.getElementById('password') as HTMLInputElement).value;

		// Send a POST request to the backend to log in
		let res = await fetch('https://gptapi.ducky.pics/login', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				username,
				password
			})
		});
		// Get staus code
		switch (res.status) {
			case 401:
				// If the status code is 401, show an error message
				alert('Wrong username or password');
				return;
			case 403:
				// If the status code is 403, show an error message
				alert('You are not allowed to log in');
				return;
			case 500:
				// If the status code is 500, show an error message
				alert('Something went wrong');
				return;
			// If the status code is 200, continue
			case 200:
				break;
			// If the status code is not 200, 401, 403 or 500, show an error message
			default:
				alert('Something went wrong');
				return;
		}
		// If the response is successful, save the token to the local storage
		localStorage.setItem('session', await res.text());
		// Redirect to the chat page
		window.location.href = '/chat';
	}

	async function loginToken() {
		// Get the token from the form
		let token = (document.getElementById('token') as HTMLInputElement).value;
		// Save the token to the local storage
		localStorage.setItem('token', token);
		// Redirect to the chat page
		window.location.href = '/chat';
	}

	function checkPassword() {
		// If the username and password are not empty, enable the submit button
		if (username && password) {
			(document.getElementById('submitPassword') as HTMLButtonElement).disabled = false;
		} else {
			(document.getElementById('submitPassword') as HTMLButtonElement).disabled = true;
		}
	}

	function checkToken() {
		// The token must be an openai token
		// Here is the regex to check if the token is valid
		let regex = /^sk-[a-zA-Z0-9]{48}$/;
		// If the token is valid, enable the submit button
		if (regex.test(token)) {
			(document.getElementById('submitToken') as HTMLButtonElement).disabled = false;
		} else {
			(document.getElementById('submitToken') as HTMLButtonElement).disabled = true;
		}
	}
</script>

<!-- This is the page when the user is not logged in -->
<p
	class="mb-4 mt-4 text-center text-2xl font-bold text-gray-800 dark:text-gray-200 md:text-3xl lg:text-4xl"
>
	You are not logged in. Please log in to continue.
</p>

<p class="text-center text-xs">
	Log in with an account or input an OpenAI API token to continue.
	<!-- This is the login form -->
</p>
<div class="flex flex-wrap items-center justify-center rounded-lg p-4">
	<div
		class="m-4 flex w-full flex-col items-center justify-center rounded-lg border-2 border-gray-400 bg-gray-200 p-4 shadow-md dark:bg-gray-800 md:w-1/2 lg:w-1/3"
	>
		<p>Log in with a username and password</p>
		<label for="username">Username</label>
		<input
			type="text"
			name="username"
			id="username"
			class="w-full rounded-md bg-gray-300 text-slate-800 dark:bg-gray-700 dark:text-gray-200"
			placeholder="Username"
			bind:value={username}
			on:input={checkPassword}
			required
		/>
		<label for="password">Password</label>
		<input
			type="password"
			name="password"
			id="password"
			class="w-full rounded-md bg-gray-300 text-slate-800 dark:bg-gray-700 dark:text-gray-200"
			placeholder="Password"
			bind:value={password}
			on:input={checkPassword}
			required
		/>
		<!-- Optionally a first-time token to then create the password -->
		<a href="/register" class=" border-blue-400 p-2 text-sm text-blue-400  hover:text-blue-500"
			>Register</a
		>
		<!-- <label for="token">Register token</label>
		<input
			type="password"
			name="registerToken"
			id="registerToken"
			class="w-full rounded-md bg-gray-300 text-slate-800 dark:bg-gray-700 dark:text-gray-200"
			placeholder="xxxx-xxxx-xxxx-xxxx"
			bind:value={registerToken}
			on:input={checkPassword}
		/> -->

		<button
			type="submit"
			id="submitPassword"
			on:click={loginPassword}
			class="mt-4 w-full rounded-lg bg-blue-400 p-2 px-4 text-slate-800 disabled:cursor-not-allowed disabled:opacity-50"
			disabled
		>
			Log in
		</button>
	</div>
	<p class="w-full text-center md:hidden">Or</p>
	<div
		class="m-4 flex w-full flex-col items-center justify-center rounded-lg border-2 border-gray-400 bg-gray-200 p-4 shadow-md dark:bg-gray-800 md:w-1/2 lg:w-1/3"
	>
		<p>
			Login using a <a
				href="https://platform.openai.com/account/api-keys"
				target="_blank"
				class="text-blue-400">token</a
			>
		</p>
		<label for="token">Token</label>
		<input
			type="text"
			name="token"
			id="token"
			class="w-full rounded-md bg-gray-300 text-slate-800 dark:bg-gray-700 dark:text-gray-200"
			placeholder="Token"
			bind:value={token}
			on:input={checkToken}
			required
		/>
		<button
			type="submit"
			id="submitToken"
			on:click={loginToken}
			class="mt-4 w-full rounded-lg bg-blue-400 p-2 px-4 text-slate-800 disabled:cursor-not-allowed disabled:opacity-50"
			disabled
		>
			Log in
		</button>
	</div>
</div>
