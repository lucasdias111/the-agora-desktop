<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fetch } from "@tauri-apps/plugin-http";
  import { listen } from "@tauri-apps/api/event";

  interface WebSocketMessage {
    type: "SEND_MESSAGE";
    toUserId: string;
    message: string;
  }

  let messages = $state([
    {
      message: "",
    },
  ]);
  let messageInput = $state("");
  let socket: WebSocket | null = null;

  async function sendMessage(toUserId: string, messageText: string) {
    try {
      await invoke("send_message_to_user", {
        toUserId: "lucas2",
        messageText: "Hello!",
      });
      messageInput = "";
      console.log("Message sent successfully");
    } catch (error) {
      console.error("Failed to send message:", error);
    }
  }

  function handleSendMessage(event: SubmitEvent) {
    event.preventDefault(); // Prevent form reload
    sendMessage("lucas2", messageInput);
  }

  listen("ws-message", (event) => {
    console.log(event.payload);
  });

  async function connectWebSocket() {
    try {
      const token = await invoke("get_stored_token");

      if (!token) {
        console.error("No token found. Please login first.");
        return;
      }

      await invoke("connect_websocket", {
        token: token,
      });
    } catch (error) {
      console.error("WebSocket connection failed:", error);
    }
  }

  function handleLogin(event: SubmitEvent) {
    event.preventDefault();
    login("lucas", "password");
  }

  async function login(username: String, password: String) {
    try {
      const result = await invoke("login", {
        username: "lucas",
        password: "password",
      });
      console.log(result);
      return true;
    } catch (error) {
      console.error("Login failed:", error);
      return false;
    }
  }
</script>

<main>
  <div class="container">
    <h1>agora</h1>
    <form class="row" onsubmit={handleLogin}>
      <input type="text" />
      <input type="password" />
      <button type="submit">Login</button>
    </form>
    <br />
    <div class="row">
      <button type="submit" onclick={connectWebSocket}>Connect</button>
    </div>
    <div class="row">
      <ul>
        {#each messages as message}
          <li>{message.message}</li>
        {/each}
      </ul>
    </div>
    <form class="row" onsubmit={handleSendMessage}>
      <input bind:value={messageInput} />
      <button type="submit">Send Message</button>
    </form>
  </div>
</main>

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

  h1 {
    text-align: center;
  }

  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
