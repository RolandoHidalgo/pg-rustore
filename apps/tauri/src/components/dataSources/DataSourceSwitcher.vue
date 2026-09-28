<script setup lang="ts">

import {SidebarMenu, SidebarMenuButton, SidebarMenuItem} from "@/components/ui/sidebar";

import {
  Tooltip, TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import {ChevronsUpDown, DatabaseZap, Plus, Unplug, Star, Pencil, Trash} from 'lucide-vue-next'
import {computed, onMounted, ref} from "vue";
import {DataSource} from "@/types";
import {useDatasourceStore} from "@/stores/datasourceStore.ts";
import {useAppStore} from "@/stores/appStore.ts";
import {Button} from "@/components/ui/button";


const dsStore = useDatasourceStore();
const appStore = useAppStore()
const datasources = computed<DataSource[]>(() => dsStore.datasources);

onMounted(async () => {
  await dsStore.loadDs(true)
  emit('change', dsStore.activeDs?.name ?? '')
})


const currentBin = computed(() => {
  return dsStore.activeDs?.bin ? parseBinVersion(dsStore.activeDs?.bin) : '';
})

function parseBinVersion(bin:string){
  if(bin.includes('PostgreSQL')){
    return `PostgreSQL v${bin.split('PostgreSQL')[1].split('/')[1]}`
  }
  return `PostgreSQL v${bin.split('pgrustore/bins/')[1].split("/")[1]}`
}

const emit = defineEmits<{
  change: [ds: string]
}>()
const handleActiveDs = (ds: DataSource) => {
  dsStore.activeDs = ds
  emit('change', ds.name)
}

function setDefaultDatasource(dsName: string) {
  // dataSourceStore.defaultDs = dsName;
  // window.electron.setDefaultDatasource(dsName)
}

async function deleteDs(name: string) {
  await dsStore.deleteDatasource(name);
}
 function getbin() {
  appStore.openBinaries()
}


const isOpen = ref(false)
</script>

<template>
  <SidebarMenu>
    <SidebarMenuItem>
      <DropdownMenu v-model:open="isOpen">
        <DropdownMenuTrigger as-child>
          <SidebarMenuButton
              size="lg"
              class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
          >
            <div
                class="flex aspect-square size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground">
              <component :is="Unplug" class="size-4"/>
            </div>
            <div class="grid flex-1 text-left text-sm leading-tight">
                <span class="truncate font-semibold capitalize">
                  {{ dsStore.activeDs?.name }}
                </span>
              <span class="truncate text-xs">{{ currentBin }}</span>
            </div>
            <ChevronsUpDown class="ml-auto"/>
          </SidebarMenuButton>
        </DropdownMenuTrigger>
        <DropdownMenuContent
            class="w-[--reka-dropdown-menu-trigger-width] min-w-56 rounded-lg"
            align="start"
            side="bottom"
            :side-offset="4"
        >
          <DropdownMenuLabel class="text-xs text-muted-foreground">
            Datasources
          </DropdownMenuLabel>
          <DropdownMenuItem
              v-for="(ds) in datasources"
              :key="ds.name"
              class="gap-2 p-2"
              @click="handleActiveDs(ds)"

          >

            <div class="flex size-6 items-center justify-center rounded-sm border">
              <component :is="Unplug" class="size-4 shrink-0"/>
            </div>
            {{ ds.name }}@{{ ds.host }}
            <!--                        <Button-->
            <!--                          variant="outline"-->
            <!--                          class="h-6 w-4.5 rounded-sm justify-self-end ml-auto"-->
            <!--                          @click.stop="deleteDs(ds.name)"-->
            <!--                          v-if="dsStore.activeDs?.name !== ds.name">-->
            <!--                          <Trash class="size-4 text-destructive" />-->
            <!--                        </Button>-->
            <TooltipProvider disable-closing-trigger>
              <Tooltip>
                <TooltipTrigger as-child>
                  <Button
                      variant="outline"
                      class="h-6 w-4.5 rounded-sm justify-self-end ml-auto"
                      @click.stop="deleteDs(ds.name)"
                      v-if="dsStore.activeDs?.name !== ds.name && dsStore.defaultDsName!==ds.name">
                    <Trash class="size-4 text-destructive"/>
                  </Button>
                </TooltipTrigger>
                <TooltipContent variant="destructive">
                  <p>Delete datasource</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>

          </DropdownMenuItem>
          <DropdownMenuSeparator/>
          <DropdownMenuLabel class="text-xs text-muted-foreground">
            Opciones del datasource
          </DropdownMenuLabel>
          <DropdownMenuItem class="gap-2 p-2" v-if="dsStore.activeDs?.name " @click="appStore.openRestore(dsStore.activeDs.name)">
            <div class="flex size-6 items-center justify-center rounded-md border bg-background">
              <DatabaseZap class="size-4"/>
            </div>
            <div class="font-medium text-muted-foreground">
              Restore
            </div>
          </DropdownMenuItem>
          <DropdownMenuItem class="gap-2 p-2" @click="setDefaultDatasource(dsStore.activeDs.name)"
                            v-if="dsStore.activeDs?.name  && dsStore.activeDs?.name !== dsStore.defaultDsName">
            <div class="flex size-6 items-center justify-center rounded-md border bg-background">
              <Star class="size-4"/>
            </div>
            <div class="font-medium text-muted-foreground">
              Set default
            </div>
          </DropdownMenuItem>

          <DropdownMenuItem class="gap-2 p-2" @click="appStore.openDataSourceForm()">
            <div class="flex size-6 items-center justify-center rounded-md border bg-background">
              <Plus class="size-4"/>
            </div>
            <div class="font-medium text-muted-foreground">
              Add datasource
            </div>
          </DropdownMenuItem>
          <DropdownMenuItem class="gap-2 p-2" @click="getbin">
            <div class="flex size-6 items-center justify-center rounded-md border bg-background">
              <Plus class="size-4"/>
            </div>
            <div class="font-medium text-muted-foreground">
              Binarios
            </div>
          </DropdownMenuItem>
          <DropdownMenuItem class="gap-2 p-2" @click="appStore.openDataSourceForm(dsStore.activeDs)"
                            v-if="dsStore.activeDs?.name">
            <div class="flex size-6 items-center justify-center rounded-md border bg-background">
              <Pencil class="size-4"/>
            </div>
            <div class="font-medium text-muted-foreground">
              Edit datasource
            </div>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

    </SidebarMenuItem>
  </SidebarMenu>

</template>

<style scoped>

</style>