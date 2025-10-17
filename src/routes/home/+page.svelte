<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import Sidebar from "$lib/components/UsersSidebar.svelte";
  import ChatArea from "$lib/components/ChatWindow.svelte";
  import Header from "$lib/components/MainHeader.svelte";

  let messages = $state([
    {
      message: "",
    },
  ]);
  let messageInput = $state("");
  let users = $state<string[]>([]);
  let chatArea: ChatArea;

  onMount(() => {
    connectWebSocket();
    getAllUsersForServer().then((fetchedUsers) => {
      users = fetchedUsers;
    });
  });

  async function getAllUsersForServer(): Promise<string[]> {
    try {
      return await invoke("get_all_users_for_server");
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

  listen<string>("ws_login", (event) => {
    users = [...users, event.payload];
  });

  listen("ws_logout", (event) => {
    users = users.filter(user => user !== event.payload);
  });

  listen("ws_message_received", (event) => {
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

  function handleUserSelected(userId: number) {
    chatArea?.loadMessagesForUser(userId);
  }
</script>

<div class="app">
  <Header />
  <div class="content">
    <Sidebar {users} onUserSelected={handleUserSelected} />
    <ChatArea
      bind:this={chatArea}
      {messages}
      {messageInput}
      onMessageInputChange={(value) => (messageInput = value)}
      onSendMessage={handleSendMessage}
    />
  </div>
</div>

<style>
  .app {
    height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
</style>
