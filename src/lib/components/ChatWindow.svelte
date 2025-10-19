<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {listen} from "@tauri-apps/api/event";
    import {currentUser} from "$lib/stores/userStores";
    import {onMount, tick} from "svelte";
    import type {User} from "$lib/models/User";

    interface ChatMessage {
        id: number | null;
        fromUserId: number;
        toUserId: number;
        message: string;
        isRead: boolean;
        isEdited: boolean;
        createdAt: string | null;
    }

    interface Props {
        messageInput: string;
        onMessageInputChange: (value: string) => void;
    }

    let selectedUser = $state<User | null>(null);
    let messages = $state<ChatMessage[]>([]);
    let {messageInput, onMessageInputChange}: Props = $props();
    let messagesContainer: HTMLDivElement;

    $effect(() => {
        messages;
        tick().then(() => {
            if (messagesContainer) {
                messagesContainer.scrollTop = messagesContainer.scrollHeight;
            }
        });
    });

    onMount(() => {
        const unlisten = listen<ChatMessage>("ws_message_received", (event) => {
            let incomingMsg: ChatMessage = event.payload;
            if (selectedUser && incomingMsg.fromUserId === selectedUser.id) {
                messages = [...messages, incomingMsg];
            }
        });

        return () => {
            unlisten.then(fn => fn());
        };
    });

    export async function selectUser(user: User) {
        selectedUser = user;
        await loadMessagesForUser(user.id);
    }

    async function loadMessagesForUser(userId: number) {
        if (!$currentUser) {
            throw new Error("No user logged in");
        }

        try {

            messages = await invoke<ChatMessage[]>("get_message_history_for_user", {
                fromUserId: $currentUser.id,
                toUserId: userId
            });
        } catch (error) {
            console.error("Failed to load messages:", error);
        }
    }


    function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        if (selectedUser && messageInput.trim()) {
            sendMessage(selectedUser.id, messageInput);
        }
    }

    async function sendMessage(toUserId: number, messageText: string) {
        try {
            await invoke("send_message_to_user", {
                toUserId: toUserId,
                messageText: messageText,
            });

            const newMessage: ChatMessage = {
                id: null, // Will be set by backend
                fromUserId: $currentUser.id,
                toUserId: toUserId,
                message: messageText,
                isRead: false,
                isEdited: false,
                createdAt: new Date().toISOString()
            };

            messages = [...messages, newMessage];

            onMessageInputChange("");
        } catch (error) {
            console.error("Failed to send message:", error);
        }
    }
</script>

<div class="chat">
    {#if selectedUser}
        <h1>{selectedUser.username}</h1>
    {:else}
        <h1>No user selected</h1>
    {/if}

    <div class="messages" bind:this={messagesContainer}>
        {#each messages as message}
            <div class="message" class:sent={message.fromUserId === $currentUser?.id}>
                {message.message}
            </div>
        {/each}
    </div>

    <form class="input-area" onsubmit={handleSubmit}>
        <input
                value={messageInput}
                oninput={(e) => onMessageInputChange(e.currentTarget.value)}
                placeholder="Type a message..."
                disabled={!selectedUser}
        />
        <button type="submit" disabled={!selectedUser}>Send</button>
    </form>
</div>

<style>
    .chat {
        flex: 1;
        display: flex;
        flex-direction: column;
    }

    .messages {
        flex: 1;
        padding: 20px;
        overflow-y: auto;
        scroll-behavior: smooth;
    }
    .message {
        padding: 10px;
        margin-bottom: 10px;
        background-color: #2a2a2a;
        border-radius: 4px;
    }

    .message.sent {
        background-color: #1e5a8e;
        margin-left: auto;
        max-width: 70%;
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

    h1 {
        text-align: center;
    }
</style>