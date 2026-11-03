import { ask, endChat, newChat } from "./turn.ts";

const prompt = process.argv.slice(2).join(" ").trim();
if (!prompt) {
  console.error('Usage: pnpm --filter client ask "Greet Ada, excitedly"');
  process.exit(1);
}

const chat = newChat();
try {
  await ask(chat, prompt);
} finally {
  endChat(chat);
}
