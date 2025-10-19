<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import Sidebar from "$lib/components/UsersSidebar.svelte";
  import ChatArea from "$lib/components/ChatWindow.svelte";
  import Header from "$lib/components/MainHeader.svelte";
  import type {User} from "$lib/models/User";
  import { currentUser } from '$lib/stores/userStores';

  let messageInput = $state("");
  let chatArea: ChatArea;

  onMount(() => {
    getCurrentUser();
    connectWebSocket();
  });

  async function getCurrentUser() {
    try {
      const user = await invoke<User>("get_current_user");
      currentUser.set(user);
    } catch (error) {
      console.error("Error getting current user:", error);
    }
  }

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

  function handleUserSelected(user: User) {
    chatArea?.selectUser(user);
  }
</script>

<div class="app">
  <Header />
  <div class="content">
    <Sidebar onUserSelected={handleUserSelected} />
    <ChatArea
            bind:this={chatArea}
            {messageInput}
            onMessageInputChange={(value) => (messageInput = value)}
    />
  </div>
</div>

<style>
  .app {
    height: 97.5vh;
    display: flex;
    flex-direction: column;
  }

  .content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
</style>