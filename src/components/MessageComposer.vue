<script setup lang="ts">
import { ref, nextTick } from "vue"

const emit = defineEmits<{
  send: [body:string];
}>();

const draft = ref("");
const inputRef = ref<HTMLInputElement | null>(null);
const showEmojiPicker = ref(false);

const emojiCategories = [
  {
    name: "Смайлы",
    emojis: ["😀", "😃", "😄", "😁", "😆", "😅", "😂", "🤣", "😊", "😇", "🙂", "🙃", "😉", "😌", "😍", "🥰"]
  },
  {
    name: "Жесты",
    emojis: ["👋", "🤚", "🖐", "✋", "🖖", "👌","🙏"]
  },
  {
    name: "Сердца",
    emojis: ["❤️", "💌", "💋", "💯", "💢", "💥", "💫", "💦", "💨", "💭", "💤"]
  },
  {
    name: "Предметы",
    emojis: ["🎉", "🎊", "🎁", "🎈", "🎂", "🍰", "🍕", "🍔", "🍟", "🌭"]
  }
];

function submitMessage(){
  const body = draft.value.trim();
  if(!body) return;
  emit("send", body)
  draft.value = "";
  showEmojiPicker.value = false;
}

function toggleEmojiPicker(){
  showEmojiPicker.value = !showEmojiPicker.value;
}

function insertEmoji(emoji: string){
  const input = inputRef.value;
  if (!input) {
    draft.value += emoji;
    return;
  }

  const start = input.selectionStart ?? draft.value.length;
  const end = input.selectionEnd ?? draft.value.length;

  const before = draft.value.substring(0, start);
  const after = draft.value.substring(end);

  draft.value = before + emoji + after;

  nextTick(() => {
    input.focus();
    const newPos = start + emoji.length;
    input.setSelectionRange(newPos, newPos);
  });
}

function closeEmojiPickerOnClickOutside(e: MouseEvent){
  const target = e.target as HTMLElement;
  if (!target.closest('.emoji-picker-wrapper')) {
    showEmojiPicker.value = false;
  }
}

if (typeof window !== 'undefined') {
  window.addEventListener('click', closeEmojiPickerOnClickOutside);
}

</script>

<template>

  <form
      class="composer"
      @submit.prevent="submitMessage"
  >
    <div class="emoji-picker-wrapper">
      <button
          type="button"
          class="emoji-toggle"
          @click.stop="toggleEmojiPicker"
          :class="{ 'emoji-toggle--active': showEmojiPicker }"
      >
        😊
      </button>

      <div
          v-if="showEmojiPicker"
          class="emoji-picker"
          @click.stop
      >
        <div
            v-for="category in emojiCategories"
            :key="category.name"
            class="emoji-category"
        >
          <div class="emoji-category-name">{{ category.name }}</div>
          <div class="emoji-grid">
            <button
                type="button"
                v-for="emoji in category.emojis"
                :key="emoji"
                class="emoji-item"
                @click="insertEmoji(emoji)"
            >
              {{ emoji }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <input
        ref="inputRef"
        v-model="draft"
        type="text"
        placeholder="Напишите что-то"
        autocomplete="off"
    />

    <button type="submit">Отправить</button>
  </form>

</template>

<style scoped>
.composer{
  display: flex;
  position: sticky;
  bottom: 0;
  gap: 10px;
  padding: 15px 20px;
  border-top: 1px solid #252830;
  background: #17191f;
  flex-shrink: 0;
  align-items: center;
}

.composer input{
  flex: 1;
  min-width: 0;
  padding: 11px 13px;
  border: 1px solid #343842;
  border-radius: 7px;
  color: #f2f3f5;
  background: #20232a;
  font: inherit;
}

.composer input:focus{
  border-color: #4f7fa4;
  outline: none;
}

.composer > button[type="submit"]{
  padding: 0 18px;
  height: 42px;
  border: none;
  border-radius: 7px;
  cursor: pointer;
  color: white;
  background: #386be0;
  font: inherit;
  font-weight: 600;
  flex-shrink: 0;
}

.composer > button[type="submit"]:hover{
  background: #2c5ac8;
}

.emoji-picker-wrapper{
  position: relative;
  flex-shrink: 0;
}

.emoji-toggle{
  width: 42px;
  height: 42px;
  border: 1px solid #343842;
  border-radius: 7px;
  background: #20232a;
  cursor: pointer;
  font-size: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.emoji-toggle:hover{
  background: #2a2e37;
  border-color: #4f7fa4;
}

.emoji-toggle--active{
  background: #2c5ac8;
  border-color: #386be0;
}

.emoji-picker{
  position: absolute;
  bottom: calc(100% + 10px);
  left: 0;
  width: 340px;
  max-height: 380px;
  overflow-y: auto;
  background: #20232a;
  border: 1px solid #343842;
  border-radius: 10px;
  padding: 14px;
  box-shadow: 0 -4px 20px rgba(0, 0, 0, 0.5);
  z-index: 9999;
}

.emoji-picker::-webkit-scrollbar{
  width: 6px;
}

.emoji-picker::-webkit-scrollbar-track{
  background: transparent;
}

.emoji-picker::-webkit-scrollbar-thumb{
  background: #3a3f4b;
  border-radius: 3px;
}

.emoji-category{
  margin-bottom: 16px;
}

.emoji-category:last-child{
  margin-bottom: 0;
}

.emoji-category-name{
  font-size: 11px;
  font-weight: 600;
  color: #8b90a0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 8px;
  padding-left: 2px;
}

.emoji-grid{
  display: grid;
  grid-template-columns: repeat(8, 1fr);
  gap: 2px;
}

.emoji-item{
  background: transparent;
  border: none;
  border-radius: 6px;
  padding: 6px 0;
  font-size: 20px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.15s;
}

.emoji-item:hover{
  background: #343842;
}

.emoji-item:active{
  background: #3d4352;
  transform: scale(0.95);
}
</style>