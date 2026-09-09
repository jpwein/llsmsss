<script setup lang="ts">

import type { Message} from "./types/messages.ts";
import type { User } from "./types/user.ts";
import Database from "@tauri-apps/plugin-sql"
import MessageList from "./components/MessageList.vue";
import MessageComposer from "./components/MessageComposer.vue";
import UserSwitcher from "./components/UserSwitcher.vue";
import { onMounted, ref } from "vue";
import AppHeader from "./components/AppHeader.vue";

const users = ref<User[]>([
  { id: 1, name: "Вы" },
  { id: 2, name: "Собеседник" },
]);

const currentUser = ref<User>(users.value[0]);

const messages = ref<Message[]>([]);

const status = ref("Подключение...")

let db: Database | null = null;
let nextId = 1;

function formatNow(): string {
  const d = new Date();
  const pad = (n: number) => n.toString().padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}`;
}

async function loadMessages(){
  if (!db) return;

  const loaded = await db.select<Message[]>(
      "SELECT id, author, body, created_at FROM messages ORDER BY id ASC",
  );
  messages.value = loaded;
  if (loaded.length > 0) {
    nextId = Math.max(...loaded.map(m => m.id)) + 1;
  }
}

async function sendMessage(body: string){
  const msg: Message = {
    id: nextId++,
    author: currentUser.value.name,
    body,
    created_at: formatNow(),
  };

  if (db) {
    try {
      await db.execute(
          "INSERT INTO messages (author, body) VALUES ($1, $2)",
          [msg.author, msg.body]
      );
      await loadMessages();
      return;
    } catch (err) {
      console.error("DB insert failed, falling back to memory:", err);
    }
  }

  messages.value = [...messages.value, msg];
}

function selectUser(user: User){
  currentUser.value = user;
}

onMounted(async()=>{
  try{
    db = await Database.load("sqlite:messanger.db");
    await loadMessages();
    status.value = "История сохраняется локально";
  } catch (error){
    console.error(error);
    status.value = "Демо-режим (без сохранения)"
  }
});

</script>

<template>
  <main class="app">
  <AppHeader :status="status"/>
    <section class = "chat">
      <div class="chat-info">
        <div class="chat-info-top">
          <div>
            <h2>Первый чат</h2>
            <p>локальный мессенджер</p>
          </div>
          <UserSwitcher
              :users="users"
              :current-user-id="currentUser.id"
              @select="selectUser"
          />
        </div>
      </div>
      <MessageList
          :messages="messages"
          :current-user-name="currentUser.name"/>

      <MessageComposer @send="sendMessage"/>
    </section>
  </main>
</template>

<style scoped>
/* все элементы будут использовать одну модель размера */
:global(*){
  box-sizing: border-box;
}

:global(html){
  background: #111318;
  color-scheme: dark;
}

:global(body){
  margin: 0;

  font-family:
  Inter,
  system-ui,
  -apple-system,
  BlinkMacSystemFont,
  "Segoe UI",
  sans-serif;

  color: #f2f3f5;

  background: #111318;
}

.app{
  height: 100vh;
  display: flex;
  flex-direction: column;
  /*
      запрещает всему app прокурчиваться
      Разрешим пркоурутку только для MessageList
  */
  overflow: hidden;
}


.chat{
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  /*
      Потому что chat целиком не должен прокручиаться, только
  */
  overflow: hidden;
}

.chat-info{
  padding: 20px 24px;
  border-bottom: 1px solid #292c34;
}

.chat-info-top{
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.chat-info h2 {
  margin: 0;
  font-size: 16px;
}

.chat-info p{
  margin: 5px 0 0;
  color: #8f96a3;
  font-size: 13px;
}

</style>
