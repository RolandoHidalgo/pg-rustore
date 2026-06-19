<script setup lang="ts">
import {Asset, DownloadEvent} from "@/types";
import {computed, ref} from "vue";
import {Button} from "@/components/ui/button";
import {Channel, invoke} from "@tauri-apps/api/core";
import {Progress} from '@/components/ui/progress'
import {Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemMedia, ItemTitle} from "@/components/ui/item";
import { Binary,CloudDownload,Trash2 } from 'lucide-vue-next'
import {Spinner} from "@/components/ui/spinner";

const installed = defineModel('installed')
const props = defineProps<Pick<Asset, 'browser_download_url' | 'name' | 'size'>>();
const size = computed(() => {
  return (Number(props.size) / 1048576).toFixed(2);
})
const isDownloading = ref(false);
const isRemoving = ref(false);
const prog = ref<number>(0);

const desc = computed(() => {
  return !isDownloading.value ? `${size.value} MB` : `Descargando: ${prog.value}%`
})
const currentName = computed(()=>{
  return props.name.split("pg-bin")[1]
})
async function download() {
  const onEvent = new Channel<DownloadEvent>();
  onEvent.onmessage = (message) => {
    
    switch (message.event) {
      case "started": {
        
        isDownloading.value = true;
        break;
      }
      case "progress": {
        
        prog.value = Number(Number(message.data.msg).toFixed(2))
        
        break;
      }
      case "finished": {
        
        isDownloading.value = false
        installed.value = true;
        break;
      }
    }

  };
  await invoke("download_bin", {url: props.browser_download_url, name: props.name, onEvent: onEvent,});
}

async function remove() {
  const onEvent = new Channel<DownloadEvent>();
  onEvent.onmessage = (message) => {
    
    switch (message.event) {
      case "started": {
        
        isRemoving.value = true;
        break;
      }
      case "finished": {
        
        isRemoving.value = false
        installed.value = false;
        break;
      }
    }

  };
  await invoke("remove_bin", {name: props.name, onEvent: onEvent,});
}
</script>

<template>
  <Item variant="outline" size="sm" class="py-2">
    <ItemMedia>
      <Spinner v-if="isDownloading"/>
      <Binary  v-else class="size-5 text-sky-500" />
    </ItemMedia>
    <ItemContent class="gap-0">

        <ItemTitle>{{ currentName }}</ItemTitle>


<!--      <template v-else>-->
<!--        <Progress :model-value="prog" class="w-full"/>-->

<!--      </template>-->
      <ItemDescription class="text-xs">
        {{ desc }}
      </ItemDescription>
    </ItemContent>
    <ItemActions>
      <Button
          v-if="!installed"
          variant="outline"
          size="icon"
          @click="download"
          :disabled="installed || isDownloading"
          type="button"
      >
        <CloudDownload/>
      </Button>
      <Button
          v-else

          variant="outline"
          size="icon"
          @click="remove"
          :disabled="!installed || isRemoving"
          type="button"
          class="text-destructive hover:text-destructive"
      >
        <Trash2/>
      </Button>
    </ItemActions>
    <ItemFooter v-if="isDownloading">
      <Progress :model-value="prog" class="w-full"/>
    </ItemFooter>
  </Item>
</template>

<style scoped>

</style>