<script lang="ts">
  import type {User} from "$lib/models/User";
  import {onMount} from "svelte";
  import {invoke} from "@tauri-apps/api/core";
  import {listen} from "@tauri-apps/api/event";
  import {currentUser} from "$lib/stores/userStores";

  interface Props {
    onUserSelected: (user: User) => void;
  }

  let { onUserSelected }: Props = $props();
  let users = $state<User[]>([]);

  onMount(() => {
    getAllUsersForServer().then((fetchedUsers) => {
      users = fetchedUsers.filter(u => u.id !== $currentUser?.id);
    });
  });

  async function getAllUsersForServer(): Promise<User[]> {
    try {
      return await invoke("get_all_users_for_server");
    } catch (error) {
      console.log("Error getting users");
      return [];
    }
  }

  listen<User>("ws_login", (event) => {
    users = [...users, event.payload];
  });

  listen<User>("ws_logout", (event) => {
    users = users.filter(user => user.id !== event.payload.id);
  });

</script>

<div class="sidebar">
  <h3>Users</h3>
  <ul>
    {#each users as user}
      <li>
        <button onclick={() => onUserSelected(user)}>{user.username}</button>
      </li>
    {/each}
  </ul>
</div>

<style>
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

  .sidebar button {
  width: 100%;
  padding: 10px;
  background-color: transparent;
  border: none;
  color: inherit;
  text-align: left;
  cursor: pointer;
  font: inherit;
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
</style>
