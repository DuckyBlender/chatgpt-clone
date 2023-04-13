<!-- It will show the login form and the anonymous "input token" form -->
<script lang="ts">
	import { onMount } from 'svelte';

	let token = '';
	let username = '';
	let password = '';
	let password_confirm = '';
	let registerToken = '';

	onMount(() => {
		// Check if the user is logged in
		if (localStorage.getItem('token') || localStorage.getItem('session')) {
			// If the user is logged in, redirect to the chat page
			window.location.href = '/chat';
		}
	});

	function checkForm() {
		// Get all form elements
		let username = (document.getElementById('username') as HTMLInputElement).value;
		let password = (document.getElementById('password') as HTMLInputElement).value;
		let password_confirm = (document.getElementById('password_confirm') as HTMLInputElement).value;
		let registerToken = (document.getElementById('registerToken') as HTMLInputElement).value;

		// Check if all the fields are filled
		if (!username || !password || !password_confirm || !registerToken) {
			// Make the button disabled
			(document.getElementById('submitPassword') as HTMLButtonElement).disabled = true;
			return;
		}
		// Check if the password and confirm password fields are the same
		if (password !== password_confirm) {
			// Make the button disabled
			(document.getElementById('submitPassword') as HTMLButtonElement).disabled = true;
			return;
		}

		// Check if the register token is valid using regex
		// The register token looks like this: 0000-0000-0000-0000
		if (!registerToken.match(/^[0-9]{4}-[0-9]{4}-[0-9]{4}-[0-9]{4}$/)) {
			// Make the button disabled
			(document.getElementById('submitPassword') as HTMLButtonElement).disabled = true;
			return;
		}

		// If all the checks are passed, enable the button
		(document.getElementById('submitPassword') as HTMLButtonElement).disabled = false;
	}

	async function register() {
		// Send the data to the back-end
		let res = await fetch('https://gptapi.ducky.pics/register', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				login: username,
				password: password,
				register_token: registerToken
			})
		});

		// Check if the request was successful
		if (res.status === 200) {
			// If the request was successful, redirect to the chat page and save the session which is returned
			window.location.href = '/chat';
			// Save the token in the local storage
			localStorage.setItem('session', await res.text());
		} else {
			// If the request was not successful, show an error message
			alert('Something went wrong. Please try again later.');
		}
	}
</script>

<!-- Register page -->
<div class="flex flex-wrap items-center justify-center rounded-lg p-4">
	<div
		class="m-4 flex w-full flex-col items-center justify-center rounded-lg border-2 border-gray-400 bg-gray-200 p-4 shadow-md dark:bg-gray-800 md:w-1/2 lg:w-1/3"
	>
		<p>Register with a username and password</p>
		<label for="username">Username</label>
		<input
			type="text"
			name="username"
			id="username"
			class="w-full rounded-md bg-gray-300 text-slate-800 dark:bg-gray-700 dark:text-gray-200"
			placeholder="Username"
			bind:value={username}
			on:input={checkForm}
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
			on:input={checkForm}
			required
		/>
		<label for="password_confirm">Password</label>
		<input
			type="password"
			name="password_confirm"
			id="password_confirm"
			class="w-full rounded-md bg-gray-300 text-slate-800 dark:bg-gray-700 dark:text-gray-200"
			placeholder="Password"
			bind:value={password_confirm}
			on:input={checkForm}
			required
		/>
		<label for="token">Register token</label>
		<input
			type="password"
			name="registerToken"
			id="registerToken"
			class="w-full rounded-md bg-gray-300 text-slate-800 dark:bg-gray-700 dark:text-gray-200"
			placeholder="xxxx-xxxx-xxxx-xxxx"
			bind:value={registerToken}
			on:input={checkForm}
		/>

		<button
			type="submit"
			id="submitPassword"
			on:click={register}
			class="mt-4 w-full rounded-lg bg-blue-400 p-2 px-4 text-slate-800 disabled:cursor-not-allowed disabled:opacity-50"
			disabled
		>
			Register
		</button>
		<!-- Go back to login -->
		<a href="/" class="mt-4 text-xs text-blue-400 hover:text-blue-500"
			>Nevermind, go back to login</a
		>
	</div>
</div>
