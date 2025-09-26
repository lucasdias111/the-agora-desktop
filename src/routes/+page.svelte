<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fetch } from "@tauri-apps/plugin-http";

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

  function sendMessage(toUserId: string, messageText: string) {
    if (!socket || socket.readyState !== WebSocket.OPEN) {
      console.error("WebSocket is not connected");
      return;
    }

    const message: WebSocketMessage = {
      type: "SEND_MESSAGE",
      toUserId,
      message: messageText,
    };
    console.log("Sending message:", message);
    socket.send(JSON.stringify(message));
    messageInput = "";
  }

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault(); // Prevent form reload
    sendMessage("lucas2", messageInput);
  }

  async function connect() {
    socket = new WebSocket("ws://localhost:8080/ws?userId=lucas");

    socket.addEventListener("open", (event) => {
      messages.push({
        message: "✅ Connected to server",
      });
    });
  }
</script>

<main>
  <div class="container">
    <h1>Welcome to Tauri + Svelte</h1>
    <div class="row">
      <button type="submit" onclick={connect}>Connect</button>
    </div>
    <div class="row">
      <ul>
        {#each messages as message}
          <li>{message.message}</li>
        {/each}
      </ul>
    </div>
    <form class="row" onsubmit={handleSubmit}>
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
