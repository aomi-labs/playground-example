import { ask, endChat, newChat } from "./turn.ts";

// Each inner list is one session; the prompts run in order inside it.
const sessions = [
  [
    "Which build are you?",
    "Greet Ada, excitedly",
    "What do two Nebula Lattes and a Moon Mochi cost?",
  ],
  ["Greet Bob"],
  ["Order 50 Comet Croissants", "add item C-99"],
];

for (const prompts of sessions) {
  const chat = newChat();
  try {
    for (const prompt of prompts) await ask(chat, prompt);
  } finally {
    endChat(chat);
  }
}
