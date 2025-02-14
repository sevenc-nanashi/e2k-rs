<script setup lang="ts">
import { ref } from "vue";
import { C2k } from "e2k";

const logs = ref<
  {
    query: string;
    answer: string;
  }[]
>([]);

const mainInput = ref("");

const c2kPromise = C2k.create(32);

const onSubmit = async () => {
  const c2k = await c2kPromise;
  if (!mainInput.value.trim()) return;
  logs.value.push({
    query: mainInput.value.trim(),
    answer: c2k.infer(mainInput.value.trim()),
  });
  mainInput.value = "";
};
</script>

<template>
  <h1>e2k-rs demo</h1>
  <p>
    <a href="https://github.com/sevenc-nanashi/e2k-rs">e2k-rs</a
    >のデモページです。
  </p>
  <main>
    <div v-for="(log, i) in logs" :key="i" class="log-row">
      <span>{{ log.query }}</span>
      <span> -&gt; </span>
      <span>{{ log.answer }}</span>
    </div>
    <form class="main-form" @submit.prevent="onSubmit">
      <input v-model="mainInput" type="text" class="main-input" />
      <button class="main-submit" type="submit">Submit</button>
    </form>
  </main>
</template>

<style scoped>
main {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 0.5em;
}
.main-form {
  display: flex;
  justify-content: center;
  gap: 1em;
}
.main-input {
  width: 200px;
  padding: 0.5em;
  font-size: 1em;
  border: 1px solid #ccc;
  border-radius: 0.25em;
}
.submit-row {
  display: flex;
  justify-content: center;
  gap: 1em;
}

.main-submit {
  padding: 0.5em 1em;
  font-size: 1em;
  border: 1px solid #ccc;
  border-radius: 0.25em;
  background-color: #f0f0f0;
  &:hover {
    background-color: #e0e0e0;
  }
  cursor: pointer;
}
</style>
