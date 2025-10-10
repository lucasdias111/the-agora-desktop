<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let messages = $state([
    {
      message: "",
    },
  ]);
  let messageInput = $state("");
  let users = $state<string[]>([]);

  onMount(() => {
    connectWebSocket();
    getAllUsersForServer().then(fetchedUsers => {
      users = fetchedUsers;
    })
  });

  async function getAllUsersForServer():Promise<string[]> {
    try {
      return await invoke ("get_all_users_for_server")
    } catch (error) {
      console.log("Error getting users");
      return [];
    }
  }

  async function sendMessage(toUserId: string, messageText: string) {
    try {
      await invoke("send_message_to_user", {
        toUserId: "lucas2",
        messageText: messageText,
      });
      messageInput = "";
      console.log("Message sent successfully");
    } catch (error) {
      console.error("Failed to send message:", error);
    }
  }

  function handleSendMessage(event: SubmitEvent) {
    event.preventDefault();
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
</script>

<div class="app">
  <!-- Top bar -->
  <div class="topbar">
    <h1>agora</h1>
  </div>

  <div class="content">
    <!-- Left sidebar - Users list -->
    <div class="sidebar">
      <h3>Users</h3>
      <ul>
        {#each users as user}
          <li>{user}</li>
        {/each}
      </ul>
    </div>

    <!-- Main chat area -->
    <div class="chat">
      <div class="messages">
        {#each messages as message}
          <div class="message">{message.message}</div>
        {/each}
      </div>

      <form class="input-area" onsubmit={handleSendMessage}>
        <input bind:value={messageInput} placeholder="Type a message..." />
        <button type="submit">Send</button>
      </form>
    </div>
  </div>
</div>

<style>
  .app {
    height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .topbar {
    height: 60px;
    background-color: #1a1a1a;
    display: flex;
    align-items: center;
    padding: 0 20px;
    border-bottom: 1px solid #333;
  }

  .topbar h1 {
    margin: 0;
    font-size: 24px;
  }

  .content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .sidebar {
    width: 250px;
    background-color: #2a2a2a;
    border-right: 1px solid #333;
    padding: 20px;
    overflow-y: auto;
  }

  .sidebar h3 {
    margin: 0 0 15px 0;
  }

  .sidebar ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .sidebar li {
    padding: 10px;
    margin-bottom: 5px;
    background-color: #333;
    border-radius: 4px;
    cursor: pointer;
  }

  .sidebar li:hover {
    background-color: #3a3a3a;
  }

  .chat {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .messages {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
  }

  .message {
    padding: 10px;
    margin-bottom: 10px;
    background-color: #2a2a2a;
    border-radius: 4px;
  }

  .input-area {
    display: flex;
    padding: 20px;
    gap: 10px;
    border-top: 1px solid #333;
  }

  .input-area input {
    flex: 1;
    padding: 10px;
    background-color: #2a2a2a;
    border: 1px solid #333;
    border-radius: 4px;
    color: #f6f6f6;
  }

  .input-area button {
    padding: 10px 20px;
  }
</style>