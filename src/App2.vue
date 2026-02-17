<script setup lang="ts">

import {Button} from "@/components/ui/button";
import {ref, shallowRef} from "vue";
import {Channel, invoke} from "@tauri-apps/api/core";
import {Button} from "@/components/ui/button";

const greetMsg = ref("");
const dbs = shallowRef<string[]>([]);
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", {name: name.value});
}

async function list_db() {


  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  dbs.value = await invoke("list_db", {name: 'asdasd'});
}

async function list_ds() {


  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  await invoke("list_ds");
}

async function backup() {
  const onEvent = new Channel<DownloadEvent>();
  onEvent.onmessage = (message) => {
    console.log(`got download event ${message.event}`);
  };

  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  await invoke("backup", {dbName:'sigap-junio',onEvent,});
}

</script>

<template>

  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>


    <div>
      <ul>
        <li v-for="db in dbs">
          {{ db }}
        </li>
      </ul>
      <Button @click="list_db" type="button">Listar db</Button>
      <Button @click="backup" type="button">backup</Button>

    </div>
  </main>
</template>
