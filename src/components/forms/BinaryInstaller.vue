<script setup lang="ts">
import {Button} from '@/components/ui/button'
import {useAppStore} from '@/stores/appStore'

import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger
} from '@/components/ui/sheet'
import {ref, watchEffect} from 'vue'
//import RestoreConsole from '@renderer/components/restore-console.vue'

import {useBinaryStore} from "@/stores/binaryStore.ts";
import BinaryItem from "@/components/forms/BinaryItem.vue";


const store = useAppStore()
const binaryStore = useBinaryStore();

//const onEvent = new Channel<DownloadEvent>();


watchEffect(async () => {
  if (store.isBinariesOpen) {
    
    await binaryStore.getRemoteBinaries()
  }
})

</script>

<template>
  <Sheet v-model:open="store.isBinariesOpen">
    <SheetTrigger></SheetTrigger>
    <SheetContent side="bottom" class="rounded-t-lg">
      <SheetHeader>
        <SheetTitle>Binaries</SheetTitle>
        <SheetDescription> Versiones de Postgres.</SheetDescription>
      </SheetHeader>


        <div class="flex w-full max-w-md flex-col gap-4 px-4">
          <template v-for="asset in binaryStore.remoteBinaries">
            <BinaryItem
                v-model:installed="asset.installed"
                :browser_download_url="asset.browser_download_url"
                :name="asset.name"
                :size="String(asset.size)"></BinaryItem>
          </template>

        </div>
      <SheetFooter>

      </SheetFooter>
    </SheetContent>
  </Sheet>
</template>

<style scoped></style>
