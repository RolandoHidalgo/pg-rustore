<script setup lang="ts">

import {useAppStore} from '@/stores/appStore'
import {Spinner} from '@/components/ui/spinner'
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
import {Empty,  EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle} from "@/components/ui/empty";
import {LoaderIcon} from "lucide-vue-next";



const store = useAppStore()
const binaryStore = useBinaryStore();

//const onEvent = new Channel<DownloadEvent>();
const isLoading = ref(false);

watchEffect(async () => {
  if (store.isBinariesOpen) {
    isLoading.value = true;
    await binaryStore.getRemoteBinaries()
    isLoading.value = false;
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
        <template v-if="isLoading">
          <Empty class="w-full">
            <EmptyHeader>
              <EmptyMedia variant="icon">
                <Spinner />
<!--                <LoaderIcon-->
<!--                            role="status"-->
<!--                            aria-label="Loading"-->
<!--                            class="size-4 animate-spin"-->
<!--                />-->
              </EmptyMedia>
              <EmptyTitle>Cargando pgBins</EmptyTitle>
              <EmptyDescription>
                Espere mientras se cargan los datos de los binarios remotos.
              </EmptyDescription>
            </EmptyHeader>

          </Empty>
        </template>
        <template v-else>
          <BinaryItem
              v-for="asset in binaryStore.remoteBinaries"
              :key="asset.name"
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
