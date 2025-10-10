<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from '$app/navigation';

  let username = $state("");
  let password = $state("");
  let loginResponse = $state("");

  function handleLogin(event: SubmitEvent) {
    event.preventDefault();
    const formData = new FormData(event.currentTarget as HTMLFormElement);
    const username = formData.get("username") as string;
    const password = formData.get("password") as string;
    login(username, password);
  }

  async function login(username: string, password: string) {
    try {
      const result: string = await invoke("login", {
        username: username,
        password: password,
      });
      loginResponse = result;
      goto('/home'); // Navigate to home
      return true;
    } catch (error: any) {
      loginResponse = String(error);
      return false;
    }
  }
</script>

<div class="container">
  <h1>agora</h1>
  <form class="row" onsubmit={handleLogin}>
    <input type="text" name="username" bind:value={username} />
    <input type="password" name="password" bind:value={password} />
    <button type="submit">Login</button>
  </form>
  <p>{loginResponse}</p>
</div>

<style>
  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .row {
    display: flex;
    justify-content: center;
  }

  h1, p {
    text-align: center;
  }
</style>