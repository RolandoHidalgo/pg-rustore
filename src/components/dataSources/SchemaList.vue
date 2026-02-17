<script setup lang="ts">
import { SidebarMenuAction, SidebarMenuButton, SidebarMenuItem } from '@/components/ui/sidebar'

import { DatabaseBackup, MoreHorizontal, Network } from 'lucide-vue-next'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger
} from '@/components/ui/dropdown-menu'
import { Skeleton } from '@/components/ui/skeleton'
import {useSchemas} from "@/composables/utils.ts";
import {useAppStore} from "@/stores/appStore.ts";

const props = defineProps<{ dsName: string; dbName: string }>()
const store = useAppStore();
const { schemas,loadingSchemas } = useSchemas(
  () => props.dsName,
  () => props.dbName
)

</script>

<template>
  <template v-if="!loadingSchemas">
    <SidebarMenuItem v-for="schema in schemas" :key="schema">
      <SidebarMenuButton>
        <Network class="text-blue-500"/>
        {{ schema }}
      </SidebarMenuButton>
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <SidebarMenuAction show-on-hover>
            <MoreHorizontal />
            <span class="sr-only">More</span>
          </SidebarMenuAction>
        </DropdownMenuTrigger>
        <DropdownMenuContent class="w-26 rounded-lg" side="bottom" align="end">
          <DropdownMenuItem @click="store.openBackup(props.dsName, props.dbName,schema)">
            <DatabaseBackup class="text-muted-foreground" />
            <span>Backup</span>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </SidebarMenuItem>
  </template>
  <template v-else>
    <div class="flex items-center space-x-4 mb-3">
      <Skeleton class="h-4 w-4 rounded-full" />
      <div class="space-y-2">
        <Skeleton class="h-3 w-[150px]" />
      </div>
      <Skeleton class="h-3 w-3 rounded-full ml-auto" />
    </div>
    <div class="flex items-center space-x-4 mb-3">
      <Skeleton class="h-4 w-4 rounded-full" />
      <div class="space-y-2">
        <Skeleton class="h-3 w-[150px]" />
      </div>
      <Skeleton class="h-3 w-3 rounded-full ml-auto" />
    </div>
  </template>
</template>

<style scoped></style>
