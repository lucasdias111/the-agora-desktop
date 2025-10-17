<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";

    interface Message {
        message: string;
    }

    interface Props {
        messages: Message[];
        messageInput: string;
        onMessageInputChange: (value: string) => void;
        onSendMessage: (event: SubmitEvent) => void;
    }

    export async function loadMessagesForUser(userId: number) {
        try {
            const history = await invoke("get_message_history_for_user", {fromUserId: 123456789, toUserId: userId});
        } catch (error) {
            console.error("WebSocket connection failed:", error);
        }
    }

    let {messages, messageInput, onMessageInputChange, onSendMessage}: Props = $props();
</script>

<div class="chat">
    <div class="messages">
        {#each messages as message}
            <div class="message">{message.message}</div>
        {/each}
    </div>
    <form class="input-area" onsubmit={onSendMessage}>
        <input
                value={messageInput}
                oninput={(e) => onMessageInputChange(e.currentTarget.value)}
                placeholder="Type a message..."
        />
        <button type="submit">Send</button>
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